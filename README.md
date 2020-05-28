# Reputation-likes
Reputation Likes through mixin-zomes or callable DNAs

Adapted from patterns developed by @pospigos and @guillemcordoba

**Do not install or run. Does not compile**

## Reputation Likes through mixin-zomes or callable DNAs

For more Reputation Modules [see this url.](https://sacred-capital.gitbook.io/sacred-capital/documentation/technical/reputation-data-types)

## Implementation as a Separate DNA

- Any DNA/App should be able to bridge to this 'Like' DNA for it's users to validate other users.
- Likes are always issued by a user towards a 'base entry' that is created by another user. The 'base entry' is undefined by us. It could be a post, a photo, a blog, audio file etc.
- - The maximum likes a user can attach to a base entry is 1. 

## Implementation in the Calling DNA

- Querying: A user should be able to query how many likes a base entry has received in total. Also, how many likes they have received cumulatively (?) 
- Mathematical operations possible on the number of likes: + - / * < > 
- Reputation Scores (Badges/Certificaitions etc.) calculated locally on the basis of operations mentioned above.


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
  }
```

![](https://i.imgur.com/XiFC9LF.png)





```
Entry "base" {
    base : string
}
```