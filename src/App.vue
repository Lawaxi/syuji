<template>
  <div class="container">
    <form class="row" @submit.prevent="search">
      <select v-model="selectedFont">
        <option value="1">楷书</option>
        <option value="2">行书</option>
        <option value="3">草书</option>
        <option value="4">隶书</option>
        <option value="5">篆书</option>
        <option value="8">篆刻</option>
        <option value="10">六体</option>
        <option value="12">印谱</option>
      </select>
      <select v-model="selectedAuthor0">
        <option value="all">全部</option>
        <option value="欧阳询">欧阳询</option>
        <option value="颜真卿">颜真卿</option>
        <option value="柳公权">柳公权</option>
        <option value="赵孟𫖯">赵孟𫖯</option>
        <option value="王羲之">王羲之</option>
        <option value="王献之">王献之</option>
        <option value="苏轼">苏轼</option>
        <option value="米芾">米芾</option>
        <option value="‌张芝">‌张芝</option>
        <option value="张旭">张旭</option>
        <option value="怀素">怀素</option>
        <option value="陆柬之">陆柬之</option>
        <option value="王铎">王铎</option>
        <option value="赵佶|宋徽宗">赵佶|宋徽宗</option>
        <option value="邓石如">邓石如</option>
        <option value="文徵明">文徵明</option>
        <option value="傅山">傅山</option>
        <option value="启功">启功</option>
        <option value="custom">自定义</option>
      </select>
      <input v-if="selectedAuthor0 === 'custom'" v-model="customAuthor" @input="updateCustomAuthor" placeholder="输入书家（简体字，以|分隔）" />
      <input id="sentence-input" v-model="sentence" placeholder="Enter a sentence..." />
      <button type="submit">検索する</button>
    </form>

    <div class="grid">
      <div v-for="(cell, index) in grid" :key="index" class="cell" @click="cycleImage(index)">
        <img
          v-if="cell.images.length > 0"
          :src="cell.currentImageIndex === -1 ? '/tauri.svg' : imageCache[cell.images[cell.currentImageIndex]] || '/vite.svg'"
          alt="失败"  @load="onImageLoad(cell.images[cell.currentImageIndex])"
        />
          <div class="author" v-if="cell.authors[cell.currentImageIndex]" v-show="cell.currentImageIndex !== -1">
            {{ cell.authors[cell.currentImageIndex] }}
          </div>
      </div>
    </div>
  </div>
</template>

<script>
import { ref, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import * as OpenCC from 'opencc-js';

const t2s = OpenCC.ConverterFactory(
  OpenCC.Locale.from.hk,
  OpenCC.Locale.to.cn
);

export default {
  setup() {
    const sentence = ref('');
    const selectedFont = ref('1');
    const selectedAuthor = ref('all');
    const selectedAuthor0 = ref('all');

    watch(selectedAuthor0, () => {
      if (selectedAuthor0.value !== 'custom') {
        selectedAuthor.value = selectedAuthor0.value;
        matchAuthor();
      }
    });
    
    const customAuthor = ref('');
    const updateCustomAuthor = () => {
      selectedAuthor.value = customAuthor.value;
      // 不支持实时匹配
    };

    const grid = ref(Array.from({ length: 18 }, () => ({ images: [], authors: [], currentImageIndex: 0 })));

    const search = async () => {
      grid.value = Array.from({ length: 18 }, () => ({ images: [], authors: [], currentImageIndex: 0 }));

      const characters = [...sentence.value];
      for (let i = 0; i < characters.length; i++) {
        const char = characters[i];
        const unicode = char.codePointAt(0).toString(16);

        try {
          const html = await invoke('fetch_html', { character: unicode, font: selectedFont.value });

          const authors = [];
          const regex = /<img[^>]*src="\/img\/loading\.gif"[^>]*title="[^"]+,\s*([^"]+)"/g;
          let match;
          while ((match = regex.exec(html)) !== null) {
            authors.push(t2s(match[1]));
          }

          if (authors.length > 0) {
            const pRegex = /var p='([^']+)'/;
            const mRegex = /var m='([a-f0-9]+)';/;
            const m2Regex = /var m2='([a-f0-9]+)';/;
            const grCookieRegex = /setCookie\('gr','([^']+)'/;

            const pMatch = html.match(pRegex);
            const mMatch = html.match(mRegex);
            const m2Match = html.match(m2Regex);
            const grCookieMatch = html.match(grCookieRegex);

            if (mMatch && m2Match) {
              const pParam = pMatch[1];
              const imageIds = pParam.split(',');
              const m = mMatch[1];
              const m2 = m2Match[1];
              const mParam = m + m2;
              const grCookie = grCookieMatch[1];

              const imageText = await invoke('fetch_image_urls', { p: pParam, m: mParam, grCookie });
              const imageUrls = imageText.match(/var images = \[([^\]]+)\]/)[1]
                .split(',')
                .map(url => url.trim().replace(/['"]/g, ''));

              grid.value[i].images = imageUrls;
              grid.value[i].authors = authors;
              grid.value[i].currentImageIndex = matchSingleAuthor(authors) || 0;
            } else {
              console.error('Failed to extract m or m2 from HTML');
            }
          }
        } catch (error) {
          console.error('Error fetching data:', error);
        }
      }
    };

    const matchSingleAuthor = (authors) => {
      console.log(selectedAuthor.value)
      if (selectedAuthor.value !== 'all') {
        const authorsToMatch = selectedAuthor.value.split('|');
        if(authors.length > 0){
            const firstIdx = authors.findIndex(author => {
              return authorsToMatch.some(authorToMatch => author.includes(authorToMatch));
            });
            if (firstIdx !== -1) {
              return firstIdx;
            } else {
              return -1;
            }
          }
      }
      return 0;
    };

    const matchAuthor = () => {
      if (selectedAuthor.value !== 'all') {
        const authorsToMatch = selectedAuthor.value.split('|');
        grid.value.forEach(cell => {
          if(cell.images.length > 0){
            const firstIdx = cell.authors.findIndex(author => {
              return authorsToMatch.some(authorToMatch => author.includes(authorToMatch));
            });
            if (firstIdx !== -1) {
              cell.currentImageIndex = firstIdx;
            } else {
              cell.currentImageIndex = -1;
            }
          }
        });
      }
    };

    const cycleImage = (index) => {
      const cell = grid.value[index];
      if (cell.images.length > 0) {
        const currentIdx = cell.currentImageIndex;
        if (selectedAuthor.value === 'all' || currentIdx === -1) {
          cell.currentImageIndex = (currentIdx + 1) % cell.images.length;
        } else {
          const authorsToMatch = selectedAuthor.value.split('|');
          const nextIdx = cell.authors
            .slice(currentIdx + 1)
            .findIndex(author => {
              return authorsToMatch.some(authorToMatch => author.includes(authorToMatch));
            });

          if (nextIdx !== -1) {
            cell.currentImageIndex = currentIdx + 1 + nextIdx; // 当前之后有
          } else {
            const firstIdx = cell.authors.findIndex(author => {
              return authorsToMatch.some(authorToMatch => author.includes(authorToMatch));
            }); // 循环查找
            if (firstIdx !== -1) {
              cell.currentImageIndex = firstIdx;
            }else{
              cell.currentImageIndex = (currentIdx + 1) % cell.images.length; // 都没找到则正常显示其他书家
            }
          }
        }
      }
    };

    const imageCache = ref({});
    const onImageLoad = async (imageName) => {
      if(imageName){
        if (!imageCache.value[imageName]) {
          await fetchImage(imageName);
        }
      }
    };

    const fetchImage = async (imageName) => {
      try {
        const imageData = await invoke("fetch_image", { imageName: imageName });
        const base64Image = `data:image/png;base64,${btoa(
          String.fromCharCode(...new Uint8Array(imageData))
        )}`;
        imageCache.value = { ...imageCache.value, [imageName]: base64Image }; // 更新缓存
      } catch (error) {
        console.error("Error fetching image:", error);
      }
    };

    return {
      sentence,
      selectedFont,
      selectedAuthor,
      selectedAuthor0,
      customAuthor,
      updateCustomAuthor,
      grid,
      search,
      cycleImage,
      imageCache,
      onImageLoad
    };
  },
};
</script>

<style>
.container {
  padding: 20px;
}

.row {
  display: flex;
  gap: 10px;
  margin-bottom: 20px;
}

.grid {
  display: grid;
  grid-template-columns: repeat(6, 1fr);
  grid-template-rows: repeat(3, 1fr);
  gap: 10px;
}

.cell {
  position: relative;
  width: 100%;
  padding-top: 100%; /* Maintain aspect ratio */
  background-color: #f0f0f0;
  display: flex;
  justify-content: center;
  align-items: center;
  cursor: pointer;
}

.cell img {
  max-width: 100%;
  max-height: 100%;
  position: absolute;
  top: 0;
  left: 0;
}

.author {
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  background: rgba(0, 0, 0, 0.5);
  color: white;
  font-size: 12px;
  padding: 5px;
  text-align: center;
  display: none;
  opacity: 0;
  transition: opacity 0.3s;
}

.cell:hover .author {
  display: block;
  opacity: 1;
}
</style>
<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}

</style>
<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}

</style>
