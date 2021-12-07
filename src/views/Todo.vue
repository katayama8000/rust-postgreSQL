<template>
  <v-container>
    <div class="bg-gray-100 p-2">
      <button
        class="bg-green-900 text-sm text-white font-bold py-1 px-2 rounded"
      >
        送信
      </button>
    </div>
    <input type="file" @change="fileUpload" />
    <img :src="this.urlbb" style="width: 20%" />

    <img :src="this.url[0]" style="width: 20%" />
    <img :src="this.url[1]" style="width: 20%" />
    <img :src="this.url[2]" style="width: 20%" />
    <img :src="this.url[3]" style="width: 20%" />
    <v-btn @click="aaa">aaa</v-btn>
    <v-btn @click="bbb">bbb</v-btn>
    <v-btn @click="ccc">ccc</v-btn>
  </v-container>
</template>

<script>
import { getStorage, ref } from "firebase/storage";
import { uploadBytes } from "firebase/storage";
import { getDownloadURL } from "firebase/storage";
import { getAuth } from "firebase/auth";
//import { getDoc } from "firebase/firestore";
import { setDoc } from "firebase/firestore";
import { doc } from "firebase/firestore";
import { db } from "../main.js";
//import { v4 as uuidv4 } from 'uuid';
//import { updateDoc, arrayUnion } from "firebase/firestore";

export default {
  data() {
    return {
      fileNames: [],
      file: null,
      url: [],
      loginToken: null,
      urlbb: null,
    };
  },

  async created() {
    // const auth = getAuth();
    // const user = auth.currentUser;
    // //ID取得
    // console.log(user.uid);
    // this.loginToken = user.uid
  },

  methods: {
    bbb() {
      // const { v4: uuidv4 } = require('uuid');
      // const v4_uuid = uuidv4();
      // console.log(v4_uuid);
      const storage = getStorage();
      const gsReference = ref(
        storage,
        "gs://sharing-text.appspot.com/5krpiLA2iqZgC9CFTe7T5ruKseA2/oasis.jpg"
      );
      console.log(gsReference);
      getDownloadURL(gsReference).then((url) => {
        this.urlbb = url;
        console.log(url);
      });
    },

    async ccc() {
      const auth = getAuth();
      const user = auth.currentUser;
      this.loginToken = user.uid;
      await setDoc(doc(db, "images", user.uid), {});
    },

    aaa() {
      const storage = getStorage();
      this.fileNames.forEach((name) => {
        console.log(name);
        getDownloadURL(ref(storage, this.loginToken + "/" + name)).then(
          (url) => {
            console.log(url);
            this.url.push(url);
          }
        );
      });
    },

    async fileUpload(event) {
      const auth = getAuth();
      const user = auth.currentUser;
      this.loginToken = user.uid;

      // this.fileNames.push(file.name);
      // console.log(this.fileNames);
      const file = event.target.files[0];

      await setDoc(doc(db, "images", user.uid), {
        imageName: [file.name],
      });

      // const Ref = doc(db, "images", user.uid);
      // // Atomically add a new region to the "regions" array field.
      // await updateDoc(Ref, {
      //   imageName: arrayUnion(file.name),
      // });

      const storage = getStorage();
      const storageRef = ref(storage, this.loginToken + "/" + file.name);
      const metadata = {
        contentType: "image/jpeg",
      };
      // Upload the file and metadata
      uploadBytes(storageRef, file, metadata);
    },

  },
};
</script>