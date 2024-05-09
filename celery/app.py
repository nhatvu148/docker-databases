from tasks import add, reverse
from celery.result import AsyncResult
from time import sleep

def print_result(task_id, task_name):
    print(task_name + ' Task ID: ' + task_id)
    result = AsyncResult(task_id)
    while result.status == 'PENDING':
        print(result.status)
        sleep(1)
    print(result.status)
    print(result.result)

# res_add = add.delay(123, 456)
res_add = add.apply_async((123, 456))
add_task_id= res_add.task_id
print_result(res_add.task_id, 'add')

# res_reverse = reverse.delay('Hello World')
res_reverse = reverse.apply_async(args=['Hello World'])
print_result(res_reverse.task_id, 'reverse')
