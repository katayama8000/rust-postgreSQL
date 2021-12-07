<template>
  <v-container>
    <input type="file" @change="fileUpload" /><br><br>
    <v-text-field v-model="name" label="名前" outlined clearable></v-text-field>
    <v-text-field
      v-model="university"
      label="大学"
      outlined
      clearable
    ></v-text-field>
    <v-text-field
      v-model="major"
      label="学部"
      outlined
      clearable
    ></v-text-field>
    <v-textarea
      outlined
      v-model="explain"
      name="input-7-4"
      label="自己紹介"
      value="〇〇大学の〇〇の講義で使用しました"
    ></v-textarea>

    <v-btn @click="send">firebaseに送信</v-btn>
  </v-container>
</template>

<script>
import { getAuth } from "firebase/auth";
import { doc, setDoc } from "firebase/firestore";
import { db } from "../main.js";
//画像のためのインポート
import { getStorage, ref } from "firebase/storage";
import { uploadBytes } from "firebase/storage";
export default {
  data: () => ({
    name: null,
    university: null,
    major: null,
    explain: null,
  }),

  methods: {
     async fileUpload(event) {
      const auth = getAuth();
      const user = auth.currentUser;
      this.userId = user.uid;

      const file = event.target.files[0];
      this.fileName = file.name

      console.log(this.userId)
      console.log(file.name)

      const storage = getStorage();
      const storageRef = ref(storage, this.userId + "/" + "avatar");
      const metadata = {
        contentType: "image/jpeg",
      };
      uploadBytes(storageRef, file, metadata);
    },

    async send() {
      const auth = getAuth();
      const user = auth.currentUser;

      if (user) {
        // User is signed in, see docs for a list of available properties
        // https://firebase.google.com/docs/reference/js/firebase.User
        await setDoc(doc(db, "users", user.uid), {
          name: this.name,
          university: this.university,
          major: this.major,
          explain: this.explain,
        });
        // ...
      } else {
        alert("ログインしてください");
        // No user is signed in.
      }
    },
  },
};
</script>

<style>
</style>