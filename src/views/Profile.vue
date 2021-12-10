<template>
  <v-container>
    <v-row>
      <v-col cols="2">
        <v-badge offset-x="20" offset-y="20" overlap color="rgba(0,0,0,0)">
          <v-avatar
            size="80"
            class="mb-3 avatar"
            style="border: 1px solid blue"
          >
            <img
              :src="url"
              alt="avatarImage"
            />
          </v-avatar>
        </v-badge>
      </v-col>
      <v-col cols="12">
        <p>名前：{{ name }}</p>
        <p>大学：{{ university }}</p>
        <p>学部：{{ major }}</p>
        <p>自己紹介：{{ explain }}</p>
      </v-col>
    </v-row>
  </v-container>
</template>

<script>
import { getDoc } from "firebase/firestore";
import { doc } from "firebase/firestore";
import { db } from "../main.js";
import { getAuth } from "firebase/auth";

import { getStorage, ref } from "firebase/storage";
import { getDownloadURL } from "firebase/storage";

export default {
  data() {
    return {
      datas: [],
      name: "",
      university: "",
      major: "",
      explain: "",
      url:null
    };
  },

  async created() {
    const auth = getAuth();
    const user = auth.currentUser;
    const uid = user.uid;

    const docRef = doc(db, "users", user.uid);
    const docSnap = await getDoc(docRef);
    this.name = docSnap.data().name;
    this.university = docSnap.data().university;
    this.major = docSnap.data().major;
    this.explain = docSnap.data().explain;

     const storage = getStorage();
        await getDownloadURL(ref(storage, uid + "/" + "avatar")).then((url) => {
          //リアクティブにするには直接代入はダメ
          this.url = url
        });
  },
};
</script>