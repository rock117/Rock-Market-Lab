import web
import json
from xpinyin import Pinyin
import os

urls = (
    '/api/pinyin', 'PinyinController',
    '/api/tushare', 'TushareController'
)
app = web.application(urls, globals())
p = Pinyin()

class PinyinController:
    def GET(self):
        return 'pinyin'
    def POST(self):
        data = json.loads(web.data())
        value = data["word"]
        print(value)
        return get_first_chinese_letter(value)

def get_first_chinese_letter(word):
    result1 = p.get_pinyin(word)
    parts = list(map(lambda p: p[0], result1.split('-')))
    return ''.join(parts)


class TushareController:
    def POST(self):
        pass



if __name__ == "__main__":
    os.environ["PORT"] = "18091"
    app.run()