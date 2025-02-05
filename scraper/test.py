import requests
from bs4 import BeautifulSoup
from playsound import playsound

link = "https://lionel2.kgv.edu.hk/user/index.php?contextid=90741&roleid=0&id=4336&perpage=5000&search=&mode=1"

"""GET /user/index.php?contextid=90741&roleid=0&id=4336&perpage=5000&search=&mode=1 HTTP/1.1
Host: lionel2.kgv.edu.hk
User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:134.0) Gecko/20100101 Firefox/134.0
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8
Accept-Language: en-US,en;q=0.5
Accept-Encoding: gzip, deflate, br, zstd
Referer: https://lionel2.kgv.edu.hk/user/index.php?contextid=90741&roleid=0&id=4336&search&perpage=5000
Connection: keep-alive
Cookie: MoodleSession=ps9ufub39cthcl7hcct9n67dl4
Upgrade-Insecure-Requests: 1
Sec-Fetch-Dest: document
Sec-Fetch-Mode: navigate
Sec-Fetch-Site: same-origin
Sec-Fetch-User: ?1
Priority: u=0, i"""


headers = {
    'User-Agent': 'Mozilla/5.0 (X11; Linux x86_64; rv:134.0) Gecko/20100101 Firefox/134.0',
    'Accept': 'text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8',
    'Accept-Language': 'en-US,en;q=0.5',
    'Accept-Encoding': 'gzip, deflate, br, zstd',
    'Referer': 'https://lionel2.kgv.edu.hk/user/index.php?contextid=90741&roleid=0&id=4336&search&perpage=5000',
    'Connection': 'keep-alive',
    'Cookie': 'MoodleSession=ps9ufub39cthcl7hcct9n67dl4',
    'Upgrade-Insecure-Requests': '1',
}

req = requests.get(link, headers=headers)


text = req.text
soup = BeautifulSoup(text)

students = soup.find_all(class_="left side cell c0")
n = len(students)

output = ''

for index, box in enumerate(students):
    user_data = box.find_all('a')[0]

    link_str = str(user_data['href'])
    user_id = link_str[link_str.index('=') + 1 : link_str.index('&')]
    name = str(user_data.find('img')['alt'][11:])

    tb_link = f"https://lionel2.kgv.edu.hk/local/mis/calendar/timetable.php/{user_id}/e637b5e2f8ec8eb6c5690f745facd66c.ics"

    tb_request = requests.get(tb_link, headers=headers)
    tb_data = tb_request.text

    f = open(f'data/{user_id}.txt', 'w')
    f.write(tb_data)
    f.close()

    f = open('data/names.txt', 'a')
    f.write(name + ' # ' + user_id + '\n')
    f.close()

    print(str(index + 1) + ' / ' + str(n))


