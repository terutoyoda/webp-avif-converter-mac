// App.svelte
<script>
  import { invoke } from '@tauri-apps/api/tauri';

  let files = []; // 複数のファイルを扱うため配列に変更
  let fileInput;
  let format = 'webp';
  let outputPath = '';

  // 出力ディレクトリの選択
  async function selectOutputDirectory() {
    try {
      const result = await invoke('select_output_directory');
      outputPath = result;
      console.log(`Selected output directory: ${outputPath}`);
    } catch (error) {
      console.error(`Error selecting output directory: ${error}`);
    }
  }

  // ドラッグアンドドロップイベントの処理
  const handleDrop = async (event) => {
    console.log("Drop detected");
    event.preventDefault();
    event.stopPropagation(); // イベントの伝播を停止
    if (event.dataTransfer.files && event.dataTransfer.files.length > 0) {
      files = Array.from(event.dataTransfer.files);
      console.log("Dropped files:", files); // デバッグ用のログ
    }
  };

  const handleDragOver = (event) => {
    console.log("Drag over detected");
    event.preventDefault();
    event.stopPropagation(); // イベントの伝播を停止
  };

  // 全ファイルをクリアする関数
  function clearAllFiles() {
    files = []; // 配列を空にする
    if (fileInput) {
      fileInput.value = ''; // ファイル入力をクリア
    }
  }

  async function convertImages() {
    if (!files.length) {
      console.error('No files specified');
      return;
    }

    // 拡張子を除外したファイル名のリストを作成
    for (const file of files) {
      const reader = new FileReader();
      reader.onload = async (e) => {
        const base64 = e.target.result.split(',')[1];
        const originalExtension = file.name.split('.').pop(); // オリジナルの拡張子を取得

        try {
          const base64Data = await invoke('convert_image', {
            fileData: base64,
            format,
            originalName: file.name,
            originalExtension // このパラメータを追加
          });
          // 変換後の処理（オプショナル）
        } catch (error) {
          console.error(error);
        }
      };
      reader.readAsDataURL(file);
    }
  }
</script>

<main on:dragover|preventDefault={handleDragOver} on:drop|preventDefault={handleDrop} class="drop-area">
  <h1>Image Converter</h1>
  <input type="file" multiple bind:this={fileInput} on:change="{(event) => (files = Array.from(event.target.files))}" />
  <select bind:value="{format}">
    <option value="webp">WebP</option>
    <option value="avif">AVIF</option>
  </select>
  <div>
    <button on:click="{selectOutputDirectory}">Select Output Directory</button>
    <button on:click="{convertImages}">Convert Images</button>
    <button on:click="{clearAllFiles}">Clear All Files</button>
  </div>

  <!-- ファイル名の表示 -->
  {#if files.length}
    <div>
      <h2>Selected Files:</h2>
      <ul>
        {#each files as file}
          <li>{file.name}</li>
        {/each}
      </ul>
    </div>
  {/if}
</main>


<style>
  .drop-area {
    border: 2px dashed #ccc;
    padding: 20px;
    margin: 10px 0;
    position: relative;
    text-align: center;
    height: 70svh;
  }
  .drop-area:hover {
    background-color: #f9f9f9;
  }
</style>
