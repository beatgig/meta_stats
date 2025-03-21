import meta_stats
import json

def test_meta_stats():
    meta_client_id = meta_stats.auth.get_meta_client_id()
    print(meta_client_id)
    meta_client_secret = meta_stats.auth.get_meta_client_secret()
    print(meta_client_secret)
    assert meta_client_id
    assert meta_client_secret

    meta_access_token = meta_stats.auth.get_meta_access_token("https://graph.facebook.com/oauth/access_token", meta_client_id, meta_client_secret, "client_credentials", "v22.0")
    print(meta_access_token)
    assert meta_access_token

    meta_access_token = meta_stats.auth.get_meta_access_token("https://graph.facebook.com/oauth/access_token", meta_client_id, meta_client_secret, "client_credentials", "22.0")
    print(meta_access_token)
    assert meta_access_token

    page_info = meta_stats.facebook.get_facebook_page_info(meta_access_token, "ChachiOfficial", "v22.0")
    print(page_info)
    print(type(page_info))


    page_followers = meta_stats.facebook.get_facebook_page_followers(meta_access_token, "ChachiOfficial", "v22.0")
    print(page_followers)
    print(type(page_followers))

    page_posts = meta_stats.facebook.get_facebook_page_posts(meta_access_token, "ChachiOfficial", "v22.0")
    print(page_posts)
    print(type(page_posts))


    
    page_posts_with_summary = meta_stats.facebook.get_facebook_page_posts_with_summary(meta_access_token, "ChachiOfficial", "v22.0")
    print(page_posts_with_summary)
    print(type(page_posts_with_summary))

    page_posts_with_summary_dict = json.loads(page_posts_with_summary)
    print(page_posts_with_summary_dict)
    print(type(page_posts_with_summary_dict))
    print(page_posts_with_summary_dict["data"])
    print(type(page_posts_with_summary_dict["data"]))

    
