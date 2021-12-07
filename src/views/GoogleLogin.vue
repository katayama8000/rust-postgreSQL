<template>
  <div>
    <v-btn @click="put_1">hello1</v-btn>
    <v-btn @click="put_2">hello2</v-btn>
    <v-btn @click="put_3">hello3</v-btn>
    <v-btn @click="put_4">hello4</v-btn>
  </div>
</template>

<script>
import { getDatabase, ref, set } from "firebase/database";
import { onValue } from "firebase/database";
import { push } from "firebase/database";
export default {
  methods: {
    put_1() {
      const userId = "aaa";
      const name = "bbb";
      const email = "ccc";
      const db = getDatabase();
      set(ref(db, "users/" + userId), {
        username: name,
        email: email,
      });
    },

    put_2() {
      const db = getDatabase();
      const starCountRef = ref(db, "users/" + "aaa");
      console.log(starCountRef);
      onValue(starCountRef, (snapshot) => {
        console.log(snapshot.val());
      });
    },

    put_3() {
      // Create a new post reference with an auto-generated id
      const db = getDatabase();
      const postListRef = ref(db, "users/" + "aaa");
      const newPostRef = push(postListRef);
      set(newPostRef, {
        name: "billie",
        email: "tattu0310@gmail.com",
        // ...
      });
    },

    put_4() {
      const db = getDatabase();
      const dbRef = ref(db, "users/" + "aaa");
      onValue(dbRef,(snapshot) => {
          snapshot.forEach((childSnapshot) => {
            console.log(childSnapshot.val());
          });
        },
        {onlyOnce: true,}
      );
    },
  },
};
</script>