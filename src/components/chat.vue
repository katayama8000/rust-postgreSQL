<template>
  <v-container>
    <v-text-field
      v-model="message"
      label="メッセージ"
      outlined
      clearable
    ></v-text-field>
    <v-btn @click="put_1">送信</v-btn>
    <v-btn @click="put_2">表示</v-btn>
    {{ name }}{{ uid }}
    {{ message }}

    <ul class="messages">
      <li v-for="(data, index) in datas" :key="index" class="text-list">
         
        <v-avatar
          size="30"
          class="mb-3 avatar"
          style="border: 1px solid blue;"
        >
          <img :src="data.url" alt="avatarImage" />
        </v-avatar>
        
        <span class="text-name">{{ data.name }}</span>
        <span class="text-message"> {{ data.message }}</span>
      </li>
    </ul>
  </v-container>
</template>

<script>
import { getDatabase, ref, set } from "firebase/database";
import { onValue } from "firebase/database";
import { push } from "firebase/database";
export default {
  data() {
    return {
      message: "",
      datas: [],
    };
  },

  props: ["name", "uid", "url"],

  created() {
    //this.put_2()
  },

  methods: {
    put_1() {
      const selectedText = this.$store.state.detailText;
      const db = getDatabase();
      const postListRef = ref(db, "messages/" + selectedText);
      const newPostRef = push(postListRef);
      set(newPostRef, {
        message: this.message,
        name: this.name,
        uid: this.uid,
        url: this.url,
      });

      this.message = "";
    },

    put_2() {
      this.datas = [];
      const selectedText = this.$store.state.detailText;
      //let count = 0
      const db = getDatabase();
      const dbRef = ref(db, "messages/" + selectedText);
      onValue(
        dbRef,
        (snapshot) => {
          snapshot.forEach((childSnapshot) => {
            //this.datas[count] = childSnapshot.val();
            this.datas.push(childSnapshot.val());
            //count++
          });
        },
        { onlyOnce: true }
      );

      console.log(this.datas);
    },
  },
};
</script>

<style scoped>
li {
  list-style: none;
}

.text-list {
  padding: 10px;
  margin: 10px 20px 10px 0px;
  border-radius: 30px;
 
}

.text-list:hover {
  background: rgb(247, 245, 245);
}

.messages {
  height: 300px;
  overflow-y: scroll;
}

.text-name{
    padding-left:5px ;
}

.text-message{
    padding-left:5px ;
}

</style>
