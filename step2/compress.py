#!/usr/bin/env python
# -*- coding: utf-8 -*-

import zlib
import binascii

original_data = open('charvideo.txt' , 'rb').read()
print('orgi:length : {}'.format(len(original_data)))

compressed_data = zlib.compress(original_data)
print('compress:length : {}'.format(len(compressed_data)))

open('charvideo_cp.txt' ,'wb').write(compressed_data)

decompress_data = zlib.decompress(compressed_data)
print('uncompress:length : {}'.format(len(decompress_data)))