from random import randint
import time

class TreeNode:
    def __init__(self, value):
        self.left = None
        self.right = None
        self.val = value

class Tree:
    def __init__(self):
        self.root = None

    def addNode(self, node, value):
        if(node==None):
            self.root = TreeNode(value)
            return self.root
        else:
            if value > node.val:
                if node.right==None:
                    node.right = TreeNode(value)
                    return node.right
                else:
                    self.addNode(node.right,value)
            else:
                if value < node.val:
                    if node.left==None:
                        node.left=TreeNode(value)
                        return node.left
                    else:
                        self.addNode(node.left,value)

    def findnb(self,node,value,nb):
        if node.val==value:
            return nb
        else:
            if value>node.val:
                return self.findnb(node.right,value,nb+1)
            else:
                if value<node.val:
                    return self.findnb(node.left,value,nb+1)


def fullscan(tab,val):
    nb=0
    for i in tab:
        nb += 1
        if i == val:
            break
    return nb

def fullscan2(tab,val):
    res = filter(lambda num: num  == val, tab)
    res2 = list(res)
    return 1
    
ecartValeur = 1500000
nbelement = 500000
nbrecherche = 5000
l=[randint(1,ecartValeur) for a in range(1,nbelement)]
tree = Tree()
first = True
root = None
for val in l:
    #print(val)
    if first:
        first=False
        root = tree.addNode(None,val)
    else:
        tree.addNode(root,val)


nbresult=0
totalresult = 0
maxresult=0
minresult=1000000
start2 = time.time()
for i in range(1,nbrecherche):
    ind=randint(1,nbelement-2)
    start = time.time()
    nb = tree.findnb(root,l[ind],0)
    end = time.time()
    nbresult += 1
    totalresult += nb
    if nb>maxresult:
        maxresult=nb
    if nb<minresult:
        minresult=nb
    #print("btree :",l[ind],nb,end-start)
end2 = time.time()
print("all execution time btree:",end2-start2," nb exec moyen: ",totalresult/nbresult," max: ",maxresult," min:", minresult)


nbresult = 0
totalresult = 0
maxresult = 0
minresult=1000000
start2 = time.time()
for i in range(1,nbrecherche):
    ind=randint(1,nbelement-2)
    start = time.time()
    nb = fullscan2(l,l[ind])
    end = time.time()
    nbresult += 1
    totalresult += nb
    if nb>maxresult:
        maxresult=nb
    if nb<minresult:
        minresult=nb    
    #print("fullscan :",l[ind],nb,end-start)
end2 = time.time()
print("all execution time fullscan:",end2-start2," nb exec moyen: ",totalresult/nbresult," max: ",maxresult," min:", minresult)

# ecart 1000000
#all execution time btree: 0.08539843559265137 199999
#all execution time fullscan: 102.62298464775085 199999


#ecartValeur = 1500000
#nbelement = 500000
#nbrecherche = 5000
#all execution time btree: 0.05004405975341797  nb exec moyen:  22.880176035207043  max:  43  min: 2
#all execution time fullscan: 63.53142714500427  nb exec moyen:  221273.59131826364  max:  499701  min: 95