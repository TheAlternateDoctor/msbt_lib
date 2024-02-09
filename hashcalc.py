label = "agbMarcher_left"
hash = 0
for char in label:
    hash = hash * 0x492 + ord(char)
    print(hash)
print(str((hash & 0xFFFFFFFF) % 101))