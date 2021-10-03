# -*- coding: utf-8 -*-
from random import randint




def ordrelexi(u,v):
    """renvoie 1 si u < v, 0 si u = v, -1 si v < u"""
    n = min(len(u),len(v))
    i = 0
    while (i < n) and (u[i] == v[i]):
        i = i + 1
    print(">",i)
    if (i < n): # donc u[i] != v[i]
        if (u[i] < v[i]):
            return "<"
        else:
            return ">"
    else: 
        if (len(u) == len(v)):
            return "="
        elif (len(u) < len(v)):
            return "<"
        else:
            return ">"

f = open("liste_francais.txt","r", encoding = "ISO-8859-1")

mots = []
nbmots=0
for m in f:
    nbmots += 1
    #print(m.rstrip())
    mots.append(m.rstrip())

for n in range(0,100):
    mind1 = randint(0, nbmots)
    #mind2 = randint(mind1, nbmots)
    mind2 = mind1 + 1
    #print("mot1 {} {}  / mot2 {} {} / {}".format(mind1,mots[mind1],mind2,mots[mind2],ordrelexi(mots[mind1],mots[mind2])))
    print("mot1  {}  / mot2  {} / lo : {} / python {}".format(mots[mind1],mots[mind2],ordrelexi(mots[mind1],mots[mind2]),mots[mind1]<mots[mind2] ))





