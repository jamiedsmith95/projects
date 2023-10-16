import datetime
from time import sleep
import pandas as pd
import praw


def rConnect():
    """
    connects to reddit using praw
    """

    reddit = praw.Reddit(
        client_id="CqGc0koWHuqrFLTZpARNpw",
        client_secret="I3aPZdRMNAzfxar4REO_FXNqs1B1_g",
        user_agent="agent",
    )
    print(reddit.read_only)
    return reddit


def postInfo(redditC, subs):
    """
    takes rConnec() and subreddit name as string as arguments
    returns subinfo dictionary with submissionId as keys and postInfo as value
    """

    subInfo = {}
    for i in range(50):
        submission = redditC.subreddit('funny').random()
        sleep(1)
        if submission in subInfo:
            pass
        else:
            postInfo = {
                "Id": submission.id,
                "Author": submission.author,
                "score": submission.score,
                "Ncomnents": submission.num_comments,
                "title": submission.title,
                "text": submission.selftext,
            }
            subInfo[submission.id] = postInfo
    return subInfo


def commentGet(redditC, subId):
    """
    takes rConnect() and subId and returns dict of all comments from that submission
    """
    submission = redditC.submission(subId)
    submission.comments.replace_more(limit = 0)
    comments_queue = submission.comments[:]
    comments = {}

    comments = get_commentLevel(comments, 0, comments_queue)

    # print("returning comments")
    return comments


def comment_finder(redditC, subInfo):
    """ """
    subInfoC = {}
    count = 0
    for i in subInfo:
        print(count)
        count += 1
        i = subInfo[i]["Id"]
        print(i)

        try:
            comments = commentGet(redditC, i)
            subInfoC.update(comments)
            # print("NOERROR")
        except TypeError:
            comments = commentGet(redditC, i)
            subInfoC.update(comments)
            # print("TypeError")
            pass

        print(i)

    return subInfoC


def get_commentLevel(comments, levl, next_level):
    """
    takes the current comments dict, the current comment level, and the list comments at that level
    levels are incremented before calling the function recursively
    """

    next = []

    while next_level:
        comment = next_level.pop(0)
        comments.update(
            {
                comment.id: {
                    "level": levl,
                    "date": datetime.datetime.fromtimestamp(comment.created_utc),
                    "author": comment.author,
                    "sub_id": comment.subreddit_id,
                    "post_id": comment.submission.id,
                    "body": comment.body,
                    "score": comment.score,
                    "parentId": comment.parent_id,
                }
            }
        )
        while True:
            try:
                comment.replies.replace_more(limit=0)
                break
            except:
                print("handling replace exception")
                sleep(1)
        next.extend(comment.replies)
    if len(next):
        levl += 1
        comments = get_commentLevel(comments, levl, next)
    else:
        return comments

    return comments

if __name__ == "__main__":
    redditC = rConnect()
    subInfo = {}
    while len(subInfo) < 1010:
        subInfo.update(postInfo(redditC, "funny"))
        print(len(subInfo))
        sleep(0.2)
    print(len(subInfo))
    subInfoC = comment_finder(redditC, subInfo)
    print(len(subInfo))
    df = pd.DataFrame.from_dict(subInfo, orient="index")
    dfC = pd.DataFrame.from_dict(subInfoC, orient="index")
    df.to_csv("reddit.csv")
    dfC.to_csv("redditC.csv")
