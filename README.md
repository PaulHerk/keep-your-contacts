# Keep-Your-Contacts

The goal of this project is to make a fully functioning website with back- and frontend for storing information from your contacts. It's made to store information like where you know them from and where they live and some more.

**This is especially useful if you have a highly social environment where it's nice to know many different persons (for networking like in business management and many more).**

### Desired Features:

- [x] User authorization
- [x] Basic information storage: create, update & delete contacts
- [x] Search & filter contacts
- [x] Map to see nearby contacts
- [ ] Extract information from LinkedIn
- [ ] Idk

Now this is in very early developement stage if you want to run it locally install, run
[shuttle run](https://www.shuttle.dev/) and `bun run dev` or any other package manager
Furthermore you need a local Postgres DB with either matches the
`local_uri` in 'src/main.rs' or provide you own local_uri.

This project is additionally hosted on Vercel & Shuttle on
[Keep-Your-Contacts](https://keep-your-contacts.vercel.app/login).
Note that you should **not** insert any private or confidential
information into this application. The only thing that is encrypted is
your password.

