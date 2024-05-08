from tasks import add, reverse

add.delay(123, 456)
result = reverse.delay('Hello World')
# print(result.status)
# print(result.get())