---
id: Library_webapp
aliases: []
tags:
  - project
---

# Library_webapp

2026-03-18  

## goal

- Store the E-book, Web-Novel, Fan-fiction, Bookmarks, etc.
- Easily manage content and library as it grow in size.
- (Optional) Create chrome extension to migrate Bookmarks into website.

## stack

- JavsaScript/TypeScript
- Next.js
- React
- TailwindCSS
- shadcn/ui
- PostgreSQL
- Prisma 7
- Cloudinary

## tasks

2026-03-19
  - [x] Set up project, schema and database.
  - [x] 17:51 Add basic POST back-end method and UI for Add Entry at `components/appEntryModal.tsx`
    - [x] 02:41 Add tags
2026-03-21
  - [x] 06:12 Create EntryCard and separate it cleanly from homepage
2026-03-22
  - [x] 20:21 add URL hotlink to go to external site
  - [x] 22:06 Add basic PATCH back-end method and UI
    - [x] 22:06 Connect Edit Button to EditEntryModal
  - [x] 20:21 Create EntryCard to separate and edit cards cleanly on homepage
  - [x] 01:34 Insert Picture URL and show picture on EntryCard
  - [x] 23:15 Add basic DELETE back-end method and UI
2026-03-23
  - [x] 17:56 Search bar
    - [x] 20:08 Search by author
    - [x] 20:08 Search by tags
2026-03-26
  - [x] 12:26 Implement Github OAuth with Auth.js
    - [x] 18:04 Email and password log-in
  - [x] 17:02 Login UI to homepage
  - [x] 18:04 Register page
  - [x] 23:20 Fix Login page out of screen
  - [x] 23:21 Move top of homepage to Navbar `layout.tsx`
2026-03-27
  - [x] Hide search bar and Add entry until Logged-in
  - [x] Show login status alongside with pfp and username
  - [x] Logout
    15:42 **Fixed** When Try to AddEntry it redirect to login-screen instead of Add Entry and showing cards.
      The cause of problem is there is no `user.id` in JWT which is required to link user to the entry so the fix is add callback in JWT to include user.id in JWT and use it as .`session.user.id` for security and **do not trust** the client for session access use server side instead.

  - [x] 19:57 Fix the homepage to show only user owned entries data
  - [x] 20:17 Show welcome message, how to user the site when user hit the site at homepage before log-in
2026-03-28
  - [x] 00:58 Redesign new landing page **Sandbox ver.**
  - [x] 15:02 New login button & logout button
  - [x] 20:51 Implement Google OAuth with Auth.js
  - [x] 23:18 Remake login-page
  - [x] 23:45 Add sidebar as placeholder for now
2026-03-30
  - [x] 23:14 Remake Shelf and Entry Card to fit new style
2026-03-31
  - [x] 11:44 Move Entry page to `"/archive"`
  - [x] 11:47 Fix search function after move to `"/archive"`
  - [x] 12:57 Show total entry at landing page/terminal page
  - [x] 14:38 Add curated function
    - [x] 17:27 Add curated page and fecth data from DB
  - [x] 17:27 Fix When Edit Card, a cards move to the first spot
2026-04-01
  - [x] 13:30 Add docs tap(Note and relation feature which is quiet big of feature)
  - [x] 14:00 Change the look to be more like a terminal
  - [x] 20:49 Handle empty state
2026-04-02
  - [x] Picture upload system via Cloudinary
    - [x] 02:09 Set up Cloudinary and API route to directly upload picture to cloud and return secureURL
    - [x] 03:28 Update DB schema to link user to secureURL
    - [x] 04:11 Update method to handle new image upload
    - [x] 17:01 Add upload image UI
      [[bug-report-image-not-show-on-DB]]
2026-04-04
  - [x] 03:09 Polish upload function to handle existing URL properly
      [[bug-report-missing-image-on-save-edit]]
  - [x] 03:10 Sync DB with Cloudinary on both edit and delete the entry
2026-04-05
  - [x] 01:41 Back-end logic for multiple shelves
  - [x] 17:52 Error handling for all Back-end logic
2026-04-06
  - [x] 14:51 Fix Front-end bug
2026-04-08
  - [x] Complete front-end for multiple shelves
    - [x] 16:05 Create new shelf modal
    - [x] 16:04 Dropdown List of the shelf
    - [x] 16:05 Fix the path navigation
    - [x] Edit shelves
    - [x] Delete shelves
2026-04-11
  - [x] 16:56 Complete status header bar for landing page
  - [x] 19:37 Fix path bug where after editing or deleting entry it redirect to / instead of current path
2026-04-19
  - [x] 15:35 Complete Responsiveness
    - [x] 13:19 Landing page
    - [x] 13:43 Archive page and Curated page
    - [x] 15:19 Log-in and Register page
    - [x] 15:35 Shelf management
  - [x] 15:19 Archive tab on non session placeholder
  - [x] 15:19 Curated tab on non session placeholder
  - [x] 16:05 More category selection
  - [x] 16:05 Error handling when server actions fail
  - [x] 20:05 Login & Register Error handling (like password length too short, username too long, invalid email format)
  - [x] 20:05 Testing Auth guards to make sure user can't access server action without log-in or access other user's data
  - [x] 20:05 Testing CRUD operations to make sure they work as expected
2026-04-20
 - [x] 13:53 Implement Auth guard for `shelfID` relate action to prevent IDOR attack
  - [x] 21:58 Finish all the testing checklist
2026-04-23
  - [x] Learn about docker and containerization to prepare for deployment
2026-04-26
  - [x] Document about docker and update README for deployment
  - [ ] Update README with demo video and screenshots
  - [ ] Hosting DB on cloud and connect to the app
  - [ ] **Optional** Group search with tags when click on tags
  - [ ] **Optional** Get default title by enter the URL instead of typing it manually
  - [ ] **Optional** Delete tags form DB

### to learn more

- [x] many to many relationship in depth
- [x] prisma 7 on how to handle many to many relationship
- [x] learn more about Cloudinary

## notes

2026-03-18
  Currently still can't host database on local due to [Supabase](https://supabase.com/dashboard/org/bkfeizeagpqgwwsfiapz) free tier require IPv6 to migrate db on cloud, See more at
2026-03-23
  When delete entry tags is still sit in the Tag table
  2026-03-31
    Might keep tags in DB just disconnect from entry
2026-03-26 **fixed**
  For google OAuth due to broken screen phone I can't do 2 step verification so add google auth later
  23:13 For now if user Add Entry with out Logged-in app would crash because in schema for Entry userId is require:

## references

[Cloudinary](https://cloudinary.com/developers)
[Supabase](https://supabase.com/dashboard/org/bkfeizeagpqgwwsfiapz)
[[javascript]]
[[typescript]]
