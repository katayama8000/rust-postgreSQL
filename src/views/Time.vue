<template>
  <div>
    <Clock class="clock" />
  </div>
</template>

<script>
import Clock from "@/components/Clock";
export default {
  components: {
    Clock,
  },

  created() {
    const aaa = {
      name: "片山",
      age: 26,
    };

    console.log(aaa);

    class Person{
      constructor(name,age){
        this.name = name,
        this.age = age
      }
    }

    const bbb = new Person("片山",25)
    console.log(bbb)
  },
};
</script>

<style scoped>
.clock {
  width: 80%;
  max-width: 500px;
  margin: 30px auto;
}
</style>
 
 
<style>
html {
  font-size: 62.5%;
}

body {
  margin: 0;
}

p {
  margin: 0;
}
</style>