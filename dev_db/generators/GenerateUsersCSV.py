import string
import random


output = ""

for index in range(50):
    name = ""
    githubID = ""

    for letter_num in range(8):
        name += random.choice(string.ascii_letters)

    for num in range(12):
        githubID += random.choice(string.ascii_letters)

    format_str = '%s, %i, "%s"\n' % (name, index, githubID)
    output += format_str

f = open("Users.csv", "w")
f.write(output)
f.close()