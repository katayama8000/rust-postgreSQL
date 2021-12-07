<template>
  <v-container>
    <v-row>
      <v-col class="text-center">
        <h1>出品</h1>
      </v-col>
    </v-row>
    <v-row justify="center">
      <v-col cols="8">
        <!-- <v-file-input
          label="File input"
          filled
          prepend-icon="mdi-camera"
          type="file"
          @change="fileUpload"
        ></v-file-input> -->
        <input type="file" @change="fileUpload" /><br><br>
        <h2>教科書の紹介</h2>
        <br />
        <p>カテゴリー</p>
        <v-col class="d-flex" cols="12" sm="6">
          <v-select
            :items="selectMajor"
            label="カテゴリー"
            outlined
            @change="fetchMajor($event)"
          ></v-select>
        </v-col>
        <p>商品の状態</p>
        <v-col class="d-flex" cols="12" sm="6">
          <v-select
            :items="selectStatus"
            label="状態"
            outlined
            @change="fetchStatus($event)"
          ></v-select>
        </v-col>
        
        <!-- ----------------------------------------------------------------------------------------- -->
        <h2>教科書名と説明</h2>
        <br />
        <p>教科書名</p>
        <v-text-field
          v-model="textName"
          label="教科書名"
          outlined
          clearable
        ></v-text-field>
        <p>教科書の説明</p>
        <v-textarea
          outlined
          v-model="explain"
          name="input-7-4"
          label="説明"
          value="〇〇大学の〇〇の講義で使用しました"
        ></v-textarea>
        <h2>受け渡し方法</h2>
        <br />
        <p>大学名</p>
        <v-text-field
          v-model="uniName"
          label="大学名"
          outlined
          clearable
        ></v-text-field>
        <v-btn @click = "send">firebaseに送信</v-btn>
        <!-- <v-radio-group v-model="row" row>
          <v-radio label="大学構内" value="radio-1"></v-radio>
          <v-radio label="最寄駅" value="radio-2"></v-radio>
          <v-radio label="その他" value="radio-3"></v-radio>
        </v-radio-group>
        <v-text-field
          v-model="message4"
          label="場合わけ必要"
          outlined
          clearable
        ></v-text-field>
        <h2>販売価格 or 欲しいもの（無ければ無料）</h2>
        <br />
        <v-text-field
          v-model="message4"
          label="場合わけ必要"
          outlined
          clearable
        ></v-text-field>
      <v-row justify="center">
        <v-col cols="8">
        <v-btn class="ma-2" outlined color="indigo" block> 出品 </v-btn>
        </v-col>
      </v-row> -->
      </v-col>
    </v-row>
  </v-container>
</template>

<style>
</style>

<script>
//storageのimport
import { getStorage, ref } from "firebase/storage";
import { uploadBytes } from "firebase/storage";
//import { getDownloadURL } from "firebase/storage";

import { getAuth } from "firebase/auth";
import { collection, addDoc } from "firebase/firestore";
//import { setDoc,doc } from "firebase/firestore";
import { db } from "../main.js";
export default {
  data: () => ({
    userId:null,
    fileName:null,
    major: null,
    status: null,
    textName: null,
    explain: null,
    uniName:null,
    selectMajor: ["経営", "経済", "理工", "機械"],
    selectStatus: [
      "新品、未使用",
      "新品に近い",
      "目立った傷や汚れなし",
      "やや傷や汚れあり",
      "傷や汚れあり",
    ],
  }),

  methods: {
    async send() {
      try {
        await addDoc(collection(db, "text"), {
          userID:this.userId,
          fileName:this.fileName,
          major: this.major,
          status: this.status,
          good:0,
          textName: this.textName,
          explain: this.explain,
          uniName:this.uniName
        });
      } catch (e) {
        alert("Error adding document: ", e);
      }
    },

    async fileUpload(event) {
      const auth = getAuth();
      const user = auth.currentUser;
      this.userId = user.uid;

      const file = event.target.files[0];
      this.fileName = file.name

      console.log(this.userId)
      console.log(file.name)

      const storage = getStorage();
      const storageRef = ref(storage, this.userId + "/" + file.name);
      const metadata = {
        contentType: "image/jpeg",
      };
      uploadBytes(storageRef, file, metadata);
    },

    fetchMajor(event) {
      this.major = event;
    },

    fetchStatus(event) {
      this.status = event;
    },
  },
};
</script>