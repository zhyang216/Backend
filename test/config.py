import requests as re

url = "http://0.0.0.0:8000/api/"

session_requests = re.session()
cookies = {'user_token': '%2FNVVg+8OpJyd9NLnN5qdez74nfnY6GsLHD8S2yv0MYeSedUxHoiGHl12qBFF9DQ12Fdu87mcDJ2RjawQRAI+XTtqJQ%3D%3D'}
session_requests.cookies.set('user_token', '%2FNVVg+8OpJyd9NLnN5qdez74nfnY6GsLHD8S2yv0MYeSedUxHoiGHl12qBFF9DQ12Fdu87mcDJ2RjawQRAI+XTtqJQ%3D%3D', domain = "0.0.0.0")