from xpinyin import Pinyin
from pypinyin import pinyin
from pypinyin import pinyin, Style

p = Pinyin()
result1 = p.get_pinyin('埃斯顿')
print(p.get_pinyin('埃斯顿'))
print(p.get_pinyin('abc'))
print(p.get_pinyin('a埃斯顿'))
print(p.get_pinyin('埃斯顿d'))


 
def get_first_letter(word):
    result1 = p.get_pinyin(word)
    parts = list(map(lambda p: p[0], result1.split('-')))
    return ''.join(parts)
    
simple = get_first_letter('埃斯顿d')    
print(simple)