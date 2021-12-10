<template>
  <v-container>
    <v-text-field
      v-model="message"
      label="メッセージ"
      outlined
      clearable
    ></v-text-field>
    <v-card v-for="(data, i) in datas" :key="i">
      <v-col cols="2">
        <v-badge offset-x="20" offset-y="20" overlap color="rgba(0,0,0,0)">
          <v-avatar
            size="80"
            class="mb-3 avatar"
            style="border: 1px solid blue"
          >
            <img :src="url" alt="avatarImage" />
          </v-avatar>
        </v-badge>
      </v-col>
      {{ data.data().name }}
      <br />
      {{ data.data().message }}
      <span @click="deleteMessage(i)">[X]</span>
    </v-card>
    <v-btn @click="addMessage">send to firebase</v-btn>
    <v-btn @click="get">get</v-btn>
  </v-container>
</template>

<script>
import { db } from "../main.js";
import { collection } from "firebase/firestore";
import { getDocs } from "firebase/firestore";
import { addDoc } from "firebase/firestore";
import { doc } from "firebase/firestore";

import { getDoc } from "firebase/firestore";
import { getAuth } from "firebase/auth";

//imageのimport
import { getStorage, ref } from "firebase/storage";
import { getDownloadURL } from "firebase/storage";

export default {
  name: "chat",
  data() {
    return {
      message: "",
      datas: [],
      data_array: [],
      name: "",
      url: null,
    };
  },

  async created() {
    const docSnap = await getDocs(collection(db, "messages"));
    this.datas = docSnap.docs;

    const auth = getAuth();
    const user = auth.currentUser;
    const uid = user.uid;
    const docRef = doc(db, "users", user.uid);
    const docS = await getDoc(docRef);
    this.name = docS.data().name;
    console.log(this.name);

    const storage = getStorage();
    await getDownloadURL(ref(storage, uid + "/" + "avatar")).then((url) => {
      //リアクティブにするには直接代入はダメ
      this.url = url;
    });
  },

  methods: {
    async create() {
      const docSnap = await getDocs(collection(db, "messages"));
      this.datas = docSnap.docs;
    },
    async addMessage() {
      await addDoc(collection(db, "messages"), {
        message: this.message,
        name: this.name,
      });
      this.message = "";
      this.create();
    },

    async deleteMessage(index) {
      console.log(this.datas[index]);
    },

    async get() {
      const querySnapshot = await getDocs(collection(db, "messages"));
      let i = 0;
      querySnapshot.forEach((doc) => {
        console.log(doc.id, " => ", doc.data());
        this.data_array[i] = doc.id;
        i++;
      });

      console.log(this.data_array);
    },
  },
};
</script>