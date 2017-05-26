#!/usr/bin/python
"""
From: https://gist.github.com/DaveCTurner/8765561

Modified to print with an extra space between fields
"""

import sys
import tarfile
import hashlib

for filename in sys.argv[1:]:
  print filename
  with tarfile.open(filename, 'r') as tar:
    for tarinfo in tar:
      if tarinfo.isreg():
        flo = tar.extractfile(tarinfo) # NB doesn't really extract the file, just gives you a stream (file-like-object) for reading it
        hash = hashlib.sha1()
        while True:
          data = flo.read(2**20)
          if not data:
            break
          hash.update(data)
        flo.close()
        print hash.hexdigest(), '', tarinfo.name
