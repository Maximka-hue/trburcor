import ast
import math
from numpy import sum
import numpy as np
import os,re
import matplotlib.pyplot as plt
from pprint import pprint
import time
import pandas as pd
from pathlib import Path
#from shutil import copyfile
import shutil
ssleep = False
debuging_smth = False
info_euclid = False
"""This will have parameters paths on which from rust processed data will generate animations

trburcor/Animation -  here diferent form types will lie with their parameters
So: file_num - describe how much files were done in terminal
with dir_types - create in there 5 files corresponding to each type
(process over possible shapes, then treated datas, and then arrays in processed files)"""
def all_files_under(path):
    """Iterates through all files that are under the given path."""
    for cur_path, dirnames, filenames in os.walk(path):
        for filename in filenames:
            yield os.path.join(cur_path, filename)
            
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
fig, ax = plt.subplots(nrows=1, ncols=1,
    figsize=(8, 4))
#Configuring paths
cwd = str(Path.home()) + "/Programming_projects/RUSTprojects"# os.getcwd()
if debuging_smth:
    print("Previous working dir was: ", cwd)
new_cwd = ""
ani_path = ""
if debuging_smth:
    print("Current working directory: {0}".format(cwd))
ani_directory = os.path.join(cwd ,"trburcor","Animations")
new_cwd = os.path.join(cwd ,"trburcor")
if not os.path.exists(ani_directory):    
    os.makedirs(ani_directory)
try:
    os.chdir(new_cwd)
    new_cwd = os.getcwd()
    if debuging_smth:
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
size = []
euclid_norm = 0.0
uniform_norm = 0.0
temp_dif_colomns = 0.0
array_paths = [Path(new_cwd + "/src/treated_datas_0/paraview_datas"), Path(new_cwd + "/src/treated_datas_1/paraview_datas"),
               Path(new_cwd + "/src/treated_datas_2/paraview_datas"), Path(new_cwd + "/src/treated_datas_3/paraview_datas"),
              Path(new_cwd + "/src/treated_datas_4/paraview_datas")]
all_files=[[],[],[],[],[]]
sizes_inner = []
for j in range(len(array_paths)):
    try:
        filelist = os.listdir(array_paths[j])
        filelist = sorted(filelist, key=lambda x: int(os.path.splitext(x.split("_")[-1])[0]))
        for file in filelist:
            print(file)
            all_files[j].append(os.path.join(array_paths[j], Path(file)))
        sizes_inner.append(len(filelist))
        print("Total number of files in ", j, end ="")
        if j == 1:
            print("st", end=" ")
        else:
            print("nd", end ="")
        print(" treated datas: ", sizes_inner[j])
    except FileNotFoundError as notfound:
        print("Not found: ", notfound)
        pass
#This is how much files to animate in each file will lie)
shapes = tuple(i for i in sizes_inner)
if debuging_smth:
    print("Shapes: " , shapes , "  ", )#*(x for x in range(10)),
#Now let's transform list into numpy array of file_paths
fn = np.zeros(shape = shapes, dtype = 'U')
print(fn.shape)
time.sleep(2.5)
#Error
fn = np.asarray([np.asarray(all_files[k]) for k in range(len(all_files))])
if ssleep:
    time.sleep(2.5)
#fn.fill([  [np.asarray(all_files[k]) for k in range(len(all_files))]  ])
print("Shape of fnp: ", fn.shape)
if debuging_smth:
    print(fn)
#This will store for all files(extern 5) exact and numeric solutions/sizes(random)
arrays = [[[],[]] , [[],[]], [[],[]], [[],[]], [[],[]]]
sizes = [[[],[]] , [[],[]], [[],[]], [[],[]], [[],[]]]
for fileind , file_next in enumerate(fn):
    print(fileind)
    if ssleep:
        time.sleep(1.5)
    num_size = 0
    exact_size = 0
    #Iterate over x_u_w_* in file
    for i , xuw in enumerate(file_next):
        xuw = str(xuw)
        fpath = os.path.join(array_paths[j], xuw)
        if os.path.isfile(xuw) and Path(xuw).name.endswith('.txt'):
            df = pd.read_csv(xuw, delimiter = ",")
            if info_euclid:
                print(all(df["numv"].notnull()))
            if any(df["numv"].notnull()):
                 #print("second condition: ", all(x !=0 for x in df[" exv"]) and all(x!=0 for x in df[" numv"]))
                if debuging_smth:
                    print("Any condition: ",any(x for x in df["numv"]))
                if any(x for x in df["exv"]) or any(x for x in df["numv"]):
                    #Calculate norms
                    max_ind = (df["exv"] - df["numv"]).idxmax()
                    if info_euclid:
                        print("Maximum differece with exact and numeric solutions in raw: ",
                            max_ind)
                    #x   exv   numv
                    if info_euclid:
                        print(df.iloc[max_ind])
                    uniform_norm = abs(df["exv"].iloc[max_ind] - df["numv"].iloc[max_ind])
                    if info_euclid:
                        print("So this maximum = ", uniform_norm)
                    #Then search euclid norm = sqrt(Sum_k (Unum_k - Uexact_k)^2)
                        print("Sum is- ", pow(df["exv"] - df["numv"], 2).sum())#axis=0
                    euclid_norm = np.sqrt(pow(df["exv"] - df["numv"], 2).sum())
                    if info_euclid:
                        print("So euclid norm is ", euclid_norm)
                    #Then write in file
                    if info_euclid:
                        print(("uniform norm in {} txt file: ").format(i), uniform_norm, file = dif)
                        print(("euclid norm: in {} txt file: ").format(i), euclid_norm, file = dif)
                    #then as usual
                    if debuging_smth:
                        print("Lenght of column: ", len(df["numv"]))
                    if len(df["numv"]) > num_size:
                        num_size = len(df["numv"])
                    if len(df["exv"]) > exact_size:
                        exact_size = len(df["exv"])
                    if all(sizes[0][0] == sizes[i][0] for i in range(len(sizes[0]))) and\
                all(sizes[0][1] == sizes[i][1] for i in range(len(sizes[0]))):
                        array_one = np.asarray(df["exv"])
                        array_two = np.asarray(df["numv"])
                        arrays[fileind][0].append(array_one)
                        arrays[fileind][1].append(array_two)
                        if debuging_smth:
                            print("exact vector: ", array_one)
                            print("numeric vector: ", array_two)
    sizes[fileind][0].append(num_size)
    sizes[fileind][1].append(exact_size)
    if debuging_smth:
        print(sizes)
"""shapes = []                        
for f_ind, n in enumerate(sizes):
    shape_one = tuple(i for i in sizes[f_ind][0])
    shape_two = tuple(i for i in sizes[f_ind][1])
    shapes.append(shape_one)
    shapes.append(shape_two)
print("Shapes next will be : ", shapes)"""
#sarr = np.zeros(shape = 
kk = filter(lambda x: len(x) > 0, arrays)
print(arrays)
filter(lambda x: len(x) > 0, sizes)
new_shapes =()
for i, k in enumerate(sizes):
    print("Treated ", i , " data")
    for j in range(len(sizes[i])):
        new_shapes += tuple(i for i in sizes[i][j])   
print(new_shapes)
snum = ()
sexact =()
div_further = tuple((new_shapes[x], new_shapes[x+1]) for x in range(0, len(new_shapes)//2, 2))
print(div_further)
ll = list(map(lambda x: tuple(x[i] for i in range(len(div_further[0]))), div_further))
for k in range(len(ll)):
    print(ll[k])
    ssnum, ssexact = ll[k]
    snum += (ssnum,)
    sexact += (ssexact,)
print("Now divided num tuple: ", snum)
print("And exact tuple: ", sexact)
fall = np.zeros(shape = (len(arrays),2), dtype = 'f')
print(fall)
fs_num = np.zeros(shape = snum, dtype = 'f')
fs_num = np.asarray([np.asarray(arrays[k]) for k in range(len(arrays[0]))])
fs_exact = np.zeros(shape = sexact, dtype = 'f')
fs_exact = np.asarray([np.asarray(arrays[k]) for k in range(len(arrays[1]))])
print("Arrays with numeric/exact datas: ", arrays)
pprint("With sizes: ", sizes)
if debuging_smth:
    pprint(arrays)

#for j in 
x = np.linspace(dl, dr, sizes[0][0])
#print(x)Max
crt = "_with correction"
change_cor = False #False mean no correction
files = []
print(shape_type)
sshape_type = dir_types[shape_type].strip("/ ")
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
        #if os.path.exists(dst_param):
        try: 
            dst_file = open(dst_param, 'w')
        except EOFError as e:
            pass
        files.append(dst_file)
        src_param = Path(new_cwd + "/src/treated_datas_{0}/parameters_nf{0}.txt".format(processed - 1))
        print("Copy parameters from:",src_param)
        print("To: ", dst_param)
        print(dir_shapes, png_path)
        print(shutil.copyfile(src_param, dst_param))
        if ssleep:
            time.sleep(3) 
        #And now process the array itself
        for k in range(len(arrays[1])):
            png_path_k = png_path + str(k)
            plt.legend(["Exact solution","Numeric solution"],loc='upper left')
            plt.xlabel('Distance on x axis')
            plt.ylabel('height')           
            if change_cor:
                plt.title(out_folder_path + ' type'+ crt)
            else:
                plt.title(out_folder_path + ' type')
            plt.plot(x, arrays[0][k],'go--', linewidth=4, markersize=3, alpha = 0.7, animated ='true',
                markerfacecoloralt = 'y', fillstyle =  'full', marker = "D")
            plt.plot(x, arrays[1][k],'yo--', linewidth=2.5, markersize=3, alpha = 0.5, animated ='true',
                markerfacecoloralt = 'b', fillstyle =  'full', marker = "X")
            #plt.pause(0.1)
            try:
                print()
                #plt.savefig(png_path_k)
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
