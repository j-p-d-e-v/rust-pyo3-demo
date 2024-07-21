import asyncio

async def myfunc():
    await asyncio.sleep(1)
    print("This is myfunc async")

def main():

    async def async_main():
        await myfunc()
    
    asyncio.run(async_main())

    return "Hello From Async"