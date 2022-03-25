import string
import random

ipsums = ["Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer in dui nulla. In sit amet est sit amet lectus sodales sodales id vitae turpis."
          , "Maecenas nec egestas purus, vel egestas nisi. Fusce volutpat nec sem a finibus. Vestibulum gravida massa sit amet diam interdum, ac aliquam quam tincidunt."
          , "Suspendisse finibus est in lacinia euismod. Morbi tincidunt sem urna, nec consectetur arcu cursus eu. Cras sit amet erat sit amet quam sagittis eleifend. "
          , "Nullam mattis, ipsum in pretium efficitur, libero tortor auctor mauris, et facilisis neque orci in ante. Aliquam eget mattis massa. "
          ,"Nam interdum euismod enim, sed suscipit massa aliquam nec. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Vivamus in risus aliquam lorem ornare euismod."]


output = ""

for index in range(100):
    name = ""
    ipsum = ""
    hash = ""

    for letter_num in range(8):
        name += random.choice(string.ascii_letters)

    for num in range(2):
        ipsum += random.choice(ipsums)

    for num in range(256):
        hash += random.choice(string.ascii_letters)

    homepage = "https://drugs-on.golem.network"

    #hash(ID), name, icon, homepage, userID
    format_str = '%s, %s, %s, "%s", %i\n' % (hash, name, "", homepage, random.randint(0, 50))
    output += format_str

f = open("Images.csv", "w")
f.write(output)
f.close()