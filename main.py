#!/usr/bin/python
#_*_ encoding:utf-8 _*_

from __future__ import division

import subprocess
import re
from termcolor import colored
import requests
from lxml import etree


# Get process info
ps = subprocess.Popen(['ps', '-caxm', '-orss,comm'], stdout=subprocess.PIPE).communicate()[0].decode()
vm = subprocess.Popen(['vm_stat'], stdout=subprocess.PIPE).communicate()[0].decode()

# Iterate processes
processLines = ps.split('\n')
sep = re.compile('[\s]+')
rssTotal = 0 # kB
for row in range(1,len(processLines)):
    rowText = processLines[row].strip()
    rowElements = sep.split(rowText)
    try:
        rss = float(rowElements[0]) * 1024
    except:
        rss = 0 # ignore...
    rssTotal += rss

# Process vm_stat
vmLines = vm.split('\n')
sep = re.compile(':[\s]+')
vmStats = {}
for row in range(1,len(vmLines)-2):
    rowText = vmLines[row].strip()
    rowElements = sep.split(rowText)
    vmStats[(rowElements[0])] = int(rowElements[1].strip('\.')) * 4096


print colored("\n\n  ==============================================================================================", "blue")

print colored("||\t\t\t\t\t\t\t\t\t\t\t", "blue"),colored("\t||", "blue")
print colored("||\t\t\t\t\t\t\t\t\t\t\t", "blue"),colored("\t||", "blue")


site = "http://ipipip.kr"

r = requests.get(site)

html = etree.HTML(r.text)

xpath = '//*[@id="ip_data"]'
filtered_html = html.xpath(xpath)
#print (len(filtered_html))


public_ip = filtered_html[0].text


print colored("||\t", "blue"), colored("공인 아이피:\t\t", "yellow"), colored("{}\t\t\t\t\t\t".format(public_ip), "red") ,colored("\t||", "blue")




print colored("||\t", "blue"), colored("CONTENT\t\t", "yellow"), colored("{}\t\t\t\t\t\t\t".format(""), "red") ,colored("\t||", "blue")


#이걸 복사해서 사용하면됨.
#print colored("||\t", "blue"), colored("CONTENT\t\t", "yellow"), colored("{}\t\t\t\t\t\t\t".format(""), "red") ,colored("\t||", "blue")


#End Motd

#Show Info


print colored("||\t\t", "blue"), colored(u'고정 메모리:', 'yellow'), colored(u'\t\t%.3f GB\t%.3f GB' % ( vmStats["Pages wired down"]/1024/1024/1024, vmStats["Pages wired down"]/1024/1024/1024 ), 'cyan'), colored("\t\t\t\t||", "blue")
print colored("||\t\t", "blue"), colored(u'활성 메모리:', 'yellow'), colored(u'\t\t%.3f GB\t%.3f GB' % ( vmStats["Pages active"]/1024/1024/1024, vmStats["Pages active"]/1024/1024/1024 ), 'cyan'), colored("\t\t\t\t||", "blue")
print colored("||\t\t", "blue"), colored(u'비활성 메모리:', 'yellow'), colored(u'\t%.3f GB\t%.3f GB' % ( vmStats["Pages inactive"]/1024/1024/1024, vmStats["Pages inactive"]/1024/1024/1024 ), 'cyan'), colored("\t\t\t\t||", "blue")
print colored("||\t\t", "blue"), colored(u'여유 메모리:', 'yellow'), colored(u'\t\t%.3f GB\t%.3f GB' % ( vmStats["Pages free"]/1024/1024/1024 , vmStats["Pages free"]/1024/1024/1024 ), 'cyan'), colored("\t\t\t\t||", "blue")
#print (u'전체 메모리 (ps):\t%.3f MB\t%.3f GB' % ( rssTotal/1024/1024, rssTotal/1024/1024/1024 ))



print colored("||\t\t\t\t\t\t\t\t\t\t\t", "blue"),colored("\t||", "blue")
print colored("||\t\t\t\t\t\t\t\t\t\t\t", "blue"),colored("\t||", "blue")

print colored("  ==============================================================================================\n\n", "blue")