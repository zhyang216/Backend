import requests as re

url = "http://0.0.0.0:8000/api/"

session_requests = re.session()
cookies = {'user_token': 'TaY86rn37GGnnXrA5v6ONdimPBCBoNG12O2uZz3tqS0NlA4q8zAVMKtvJo%2Fcf6OmeLBxKhjAxqE63gr+AG%2FhkVj%2FMw%3D%3D'}
session_requests.cookies.set('user_token', 'TaY86rn37GGnnXrA5v6ONdimPBCBoNG12O2uZz3tqS0NlA4q8zAVMKtvJo%2Fcf6OmeLBxKhjAxqE63gr+AG%2FhkVj%2FMw%3D%3D', domain = "0.0.0.0")