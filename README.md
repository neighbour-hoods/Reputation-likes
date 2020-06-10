# Reputation Data Types | 'Likes'

*For more Reputation Modules [see this url.](https://sacred-capital.gitbook.io/sacred-capital/documentation/technical/reputation-data-types)*

*Design is in keeping with principles of modularity that are [discussed here.](https://hackmd.io/@TJSptuQkSdWyLgQxZrAUGg/S1RSmuptU)*

Reputation Modules are designed as separate DHT's that can bridge into any existing app. This allows a community to pick which reputation module is appropriate for them and choose between specific features. 

For example, a community can swap out a 'likes' module and instead decide to incorporate 'claps' or 'upvotes/downvotes' based on how they wish to articulate culture within their collective. 

They do so by picking a DNA template, and adding their community's unique identifier in the 'properties' field to spin-off a new DHT. 

![](https://i.imgur.com/8Aju8wB.png)


Assuming they pick the 'likes' DNA, they will also have the choice of whether:
- Likes received and given is private information i.e seen only by those who give, and those who receive.
- Likes received and given is public info
- Likes received and given cannot be seen by receiver. 

![](https://i.imgur.com/OXoYUiw.png)



## Implementation as a Separate DNA

- Any front-end module should be able to call this 'Like' DNA for it's users to validate other users.
- Likes are always issued by a user towards a 'base entry' that is created by another user. The 'base entry' is undefined by us. It could be a post, a photo, a blog, audio file etc.
- The maximum likes a user can attach to a base entry is 1. (discuss) 

## Issuing Certifications, Badges, Stars etc. 

![](https://i.imgur.com/uUyEXpF.png)


The community can use the Likes DHT to aggregate the number of likes, and issue Reputation Scores (Badges/Labels/Certifications etc.) calculated on a specific DHT. 
- Mathematical operations possible on the number of likes: + - / * < > 




Entries:

```
Entry "like" {
    struct : Like {
        base: string,
        timestamp: u64,
        auth_address: Address,
    }
}

Links {
  users -> Likes
  Base Entry -> Likes
  }
```







```
Entry "base" {
    base : Unique universal identifier for the entry that is being commented on. 
}
```

