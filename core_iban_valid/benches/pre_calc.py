import math
print(math.floor((int(math.pow(2,32) / 97))))

def short_mod97(x):
    #     const DIVISOR :u32 = 97;
    # const MULTIPLIER : u32 = 44278013; // floor(2^32 / 97)
    # // let quotient = ((x as u64 * MULTIPLIER as u64) >> 32) as u32;
    return x - (((x * 44278013 ) >> 32) * 97)

# for i in range(1000):
#     if (i % 97 ) != short_mod97(i) :
#         print(i, i%97, short_mod97(i))

print(96, 96%97, short_mod97(96), 97*((96 * 44278013 ) >> 32))
print(97, 97%97, short_mod97(97), 97*((97 * 44278013 ) >> 32))
print(98, 98%97, short_mod97(98), 97*((98 * 44278013 ) >> 32))
# print(i, i%97, short_mod97(i))