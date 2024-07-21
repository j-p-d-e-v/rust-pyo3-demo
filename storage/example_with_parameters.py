def main(arg1,**kwargs):
    print("arg1",arg1)
    print("kwargs",kwargs)
    return {
        "key1": arg1,
        "key2": kwargs,
        "key3": "Hello World"
    }