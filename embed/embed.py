from ctypes import cdll

lib = cdll.LoadLibrary("target/release/embed.dll")

lib.process()

print("done!")