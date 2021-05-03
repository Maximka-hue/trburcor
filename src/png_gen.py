import ast
import math
import numpy as np
import os,re
from IPython.display import HTML
import matplotlib.pyplot as plt
import matplotlib.animation as animation
from pprint import pprint
import time
import pandas as pd
from pathlib import Path
#from shutil import copyfile
import shutil
ssleep = True
"""This will have parameters paths on which from rust processed data will generate anmations

trburcor/Animation -  here diferent form types will lie with their parameters
So: file_num describe how much files were done in terminal
with dir_types create in there 5 files corresponding to each type
and process over possible shapes, then treated datas, and then arrays in processed files"""
def glob_re(path, regex="", glob_mask="**/*", inverse=False):
    p = Path(path)
    count=0
    if inverse:
        res = [str(f) for f in p.glob(glob_mask) if not re.search(regex, str(f))]
        count+=1
    else:
        res = [str(f) for f in p.glob(glob_mask) if re.search(regex, str(f))]
        count+=1
    return (res,count)
# First set up the figure, the axis, and the plot element we want to animate
fig, ax = plt.subplots()
#Configuring paths
cwd = os.getcwd()
new_cwd = ""
ani_path = ""
print("Current working directory: {0}".format(cwd))
ani_directory = os.path.join(cwd ,"trburcor","Animations")
new_cwd = os.path.join(cwd ,"trburcor")
if not os.path.exists(ani_directory):    
    os.makedirs(ani_directory)
try:
    os.chdir(new_cwd)
    new_cwd = os.getcwd()
    print("Directory changed: {0}".format(new_cwd))
except FileNotFoundError:
    print("Directory: {0} does not exist".format(ani_directory))
except NotADirectoryError:
    print("{0} is not a directory".format(ani_directory))
except PermissionError:
    print("You do not have permissions to change to {0}".format(ani_directory))
except OSError:
    print("Can't change the Current Working Directory")  
route = Path(new_cwd + "/src/txt_to_parse")
print("Txt files lay in: ", route)
for dirpath, _dirnames, filenames in os.walk(route):
    print('Текущий путь:', dirpath)
    print('Файлы:')
    pprint(filenames)
#Here store all example files to run with
processed_rust_files = Path(new_cwd + "/src")
(tar, c) = glob_re(processed_rust_files, regex="treated_datas_")
file_num = np.arange(1, c+1)
suffix_to_file = {0: "/_one_", 1: "/_two_", 2: "/_three", 3: "/_four", 4: "/_five"}
dir_types = ['/runge','/triangle','/gauss_wave','/sinusoid','/lines']
print(file_num)
dl=0.0
dr=0.0
equation_type = 0
shape_type = 0
#_____________________Processing datas___________________________________#
param_path = Path(new_cwd + r"/src/treated_datas_0/parameters_nf0.txt")
try:
    fp =  open(param_path, 'r')
    l = [line.strip() for line in fp]
    pprint(l)
    for ll in l:
        if "Initial type" in ll:
            res = re.sub('[^0-9]+', '', ll)
            equation_type = float(res)
            print("equation_type", equation_type)
        if "Margin domain: " in ll:
            k = [float(el) for el in ll[len('margin_domain: '):-1].replace('(','').replace(')','').replace("\\","").split(',')]
            print("Left bound", k[0])
            dl=k[0]
            print("Right bound", k[1])
            dr=k[1]
            ax.set_xlim(( k[0]-0.2, k[1]+0.2))
            #ax[1].set_xlim(( k[0]-0.2, k[1]+0.2))
        if "Initial conditions:" in ll:
            k = [float(el) for el in ll[len("Initial conditions: "):-1].replace('(','').replace("Some","").replace(')','').replace("\\","").split(',')]
            print("Center/MatExpect", k[0])
            print("Height/Dispersion", k[1])
            print("Width", k[2])
            if equation_type == 0 or equation_type == 1:
                ax.set_ylim((0, k[2]))#k[1]
            #ax[1].set_ylim((-1, k[2]))
            if equation_type == 2:
                ax.set_ylim((0, 0.002))#k[1]
            if equation_type == 3 or equation_type == 4:
                ax.set_ylim((-1, k[2]))#k[1]
        if "Initial type" in ll:
                res = re.sub('[^0-9]+', '', ll)
                shape_type = int(res)
                print("shape_type", shape_type)
except IOError:
    print ("No file")
#________________________________________________________________________#
dif_errors = []
for k in file_num:
    differ_path = Path(new_cwd + "/src/differential_errors{}.txt".format(k - 1))
    print(differ_path)
    if not os.path.exists(differ_path):    
        os.makedirs(differ_path)
    dif = open(differ_path, 'w')
    dif_errors.append(dif)
    if ssleep:
        time.sleep(2) 
sizes = [[],[]]
size = []
euclid_norm = 0.0
uniform_norm = 0.0
temp_dif_colomns = 0.0
arrays = [[],[]]
array_path = Path(new_cwd + "/src/treated_datas_0/paraview_datas")
for i,entry in enumerate(os.scandir(array_path)):
    f = os.path.join(array_path, entry)
    if os.path.isfile(f) and entry.name.endswith('.txt'):
        #print(f)
        df = pd.read_csv(f, delimiter = ",")
        print(all(df["numv"].notnull()))
        if any(df["numv"].notnull()):
            #print("second condition: ", all(x !=0 for x in df[" exv"]) and all(x!=0 for x in df[" numv"]))
            print("Any: ",any(x for x in df["numv"]))
            if any(x for x in df["exv"]) or any(x for x in df["numv"]):
                #Calculate norms
                max_ind = (df["exv"] - df["numv"]).idxmax()
                print("Maximum differece with exact and numeric solutions in raw: ",
                    max_ind)
                #x   exv   numv
                print(df.iloc[max_ind])
                uniform_norm = abs(df["exv"].iloc[max_ind] - df["numv"].iloc[max_ind])
                print("So this maximum = ", uniform_norm)
                #Then search euclid norm = sqrt(Sum_k (Unum_k - Uexact_k)^2)
                print("Sum is- ", pow(df["exv"] - df["numv"], 2).sum())#axis=0
                euclid_norm = math.sqrt(pow(df["exv"] - df["numv"], 2).sum())
                print("So euclid norm is ", euclid_norm)
                #Then write in file
                print(("uniform norm in {} txt file: ").format(i), uniform_norm, file = dif)
                print(("euclid norm: in {} txt file: ").format(i), euclid_norm, file = dif)
                #then as usual
                print("Lenght of column: ", len(df["numv"]))
                sizes[0].append(len(df["numv"]))
                sizes[1].append(len(df["exv"]))
                if all(sizes[0][0] == sizes[0][i] for i in range(len(sizes[0]))):
                    print(sizes)
                    array_one = np.asarray(df["exv"])
                    array_two = np.asarray(df["numv"])
                    arrays[0].append(array_one)
                    arrays[1].append(array_two)
                    print("exact vector: ", array_one)
                    print("numeric vector: ", array_two)
            #break
        #l = [line for line in df]
        #pprint(l)
pprint(arrays)
print("array lenght" , len(arrays[0]))
print(len(arrays[1]))
    
x = np.linspace(dl, dr, sizes[0][0])
#print(x)Max
crt = "_with correction"
change_cor = False #False mean no correction
files = []
print(shape_type)
sshape_type = dir_types[shape_type].strip("/")
#process different shapes and create directories for them
for (i, type) in enumerate(dir_types):
    out_folder_path = type
    print(type)
    dir_shapes = Path(ani_directory + out_folder_path)
    if os.path.exists(dir_shapes):
        pass
    else:
        try:
            os.mkdir(ani_directory + out_folder_path)
        #except FileExistsError as e:
        #    print('File already exists')
        except OSError as e:
            print("An error has occurred. Continuing anyways: {e}")
    if type.strip("/") != sshape_type:
        continue
    #Now process every treated_directory
    for processed in file_num:
        if change_cor:
            png_path = ani_directory + out_folder_path + suffix_to_file.get(i) + crt
        else:
            png_path = ani_directory + out_folder_path + suffix_to_file.get(i)
        dst_param = os.path.join(ani_directory + out_folder_path, r'parameters_' + str(processed) + '.txt')
        print("Copy parameters from:", dst_param)
        #if os.path.exists(dst_param):
        try: 
            dst_file = open(dst_param, 'w')
        except EOFError as e:
            pass
        files.append(dst_file)
        src_param = Path(new_cwd + "/src/treated_datas_{0}/parameters_nf{0}.txt".format(processed - 1))
        print("To: ", src_param)
        print(dir_shapes, png_path)
        print(shutil.copyfile(src_param, dst_param))
        if ssleep:
            time.sleep(4) 
        #And now process the array itself
        for k in range(len(arrays[0])):
            png_path_k = png_path + str(k)
            print(png_path_k)
            cur_image_path = os.path.join(out_folder_path, png_path_k)
            print(cur_image_path)
            plt.legend(["Exact solution","Numeric solution"],loc='upper left')
            plt.xlabel('Distance on x axis')
            plt.ylabel('height')           
            if change_cor:
                plt.title(out_folder_path + ' type'+ crt)
            else:
                plt.title(out_folder_path + ' type')
            plt.plot(x, arrays[0][i],'go--', linewidth=2, markersize=3, alpha = 0.6, animated ='false',
                markerfacecoloralt = 'b', fillstyle =  'left')
            plt.plot(x, arrays[1][i],'yo--', linewidth=3, markersize=3, alpha = 0.5, animated ='false',
                markerfacecoloralt = 'b', fillstyle =  'full')
            #plt.pause(0.1)
            try:
                plt.savefig(cur_image_path)
            except FileExistsError as e:
                print('File already exists')
            except OSError as e:
                print("Continuing anyways: {e}")
            plt.show()
    
lines=[[]]
#colour map
cmap = ["green", "blue", "red", "orange"]
for f in dif_errors:
    f.close()
for fi in files:
    fi.close()
