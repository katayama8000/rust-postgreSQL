<template>
  <div class="about">
    <h1 @click="aaa">This is an about page</h1>
  </div>
</template>

<script>
import { getAuth } from "firebase/auth";
import { doc, setDoc } from "firebase/firestore";
import { db } from "../main.js";
export default {
  methods: {
    async aaa() {
      // ログイン済みのユーザー情報があるかをチェック

      const auth = getAuth();
      const user = auth.currentUser;
      console.log(user)
       console.log(user.uid)


      if (user) {
        // User is signed in, see docs for a list of available properties
        // https://firebase.google.com/docs/reference/js/firebase.User
        await setDoc(doc(db, "users", user.uid), {
          display_name: 'ooo',
          state: "nagoya",
          country: "JP",
        });
        // ...
      } else {
        // No user is signed in.
      }
    },
  },
};
</script>
