from django.shortcuts import render

# Create your views here.

posts = [
        {
            'author': 'CoreyMs',
            'title': 'Blog Post 1',
            'content':'First post content',
            'date_posted': 'August 27, 2018'
            },
        {
            'author': 'JamieS',
            'title': 'Blog Post 2',
            'content':'Second post content',
            'date_posted': 'August 28, 2018'
            }

        ]
def home(request):
    context = {
            'posts': posts
            }
    return render(request, 'blog/home.html',context) #HttpResponse('<h1>Blog Home</h1>')

def about(request):
    return render(request, 'blog/about.html',{'title': 'About'})#HttpResponse('<h1>Blog About</h1>')
