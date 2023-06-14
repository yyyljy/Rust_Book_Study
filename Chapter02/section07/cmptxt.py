afile = "./fizzbuzz_python_jw.txt"
bfile = "./fizzbuzz_rust_jw.txt"

with open(afile, "r") as fp:
    astr = fp.read()
with open(bfile, "r") as fp:
    bstr = fp.read()

# astr = astr.strip()
# bstr = bstr.strip()

if astr == bstr:
    print("ok")
else:
    print("ng")