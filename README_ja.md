# webp/avif Image Converter for Mac

<img src="./app-icon.png" alt="webp/avif Image Converter" width="200"/><br>

## 機能

- webp/avif Image Converterは、画像ファイルをwebpやavif形式に変換するためのデスクトップアプリケーションです。このアプリケーションは、SvelteとTauriを使用して開発されています。
## インストール

webp/avif Image Converterをインストールするには、以下の手順に従ってください：

1. 最新のリリースをダウンロードします。
2. ダウンロードしたファイルを解凍し、アプリケーションを起動します。
3. npm run build, npm run tauri:buildを実行して、アプリケーションをビルドします。
4. ビルドが完了すると、src-tauri/target/bundle/macosディレクトリにアプリケーションが生成されます。

## 使い方

1. アプリケーションを開きます。
2. 「Select Output Directory」ボタンをクリックして、変換後の画像ファイルの保存先を選択します。
3. 変換したい画像ファイルをウィンドウにドラッグアンドドロップするか、「ファイルを選択」ボタンからファイルを追加します。
4. 下部のドロップダウンメニューから希望の出力フォーマット（webpまたはavif）を選択します。
5. 「Convert Images」ボタンをクリックして変換を開始します。

## 注意点

1. Macのみで動作確認しています。
2. 当アプリの使用に関連して生じたデータ損失や機能障害など、直接的または間接的ないかなる損害に対しても、当方は責任を負いません。
3. ユーザーは、本アプリを使用することによって、第三者の権利を侵害しないよう注意する責任を負います。第三者の権利侵害に関連する一切の責任は、ユーザーに帰属します。

## その他

GUIからアプリケーション実行し、上手く変換できない場合、ファイルシステムへの書き込み権限が、この実行方法では確実に与えられていない可能性があります。その場合、Terminalからの実行で、特定のセキュリティ制限が緩和される場合があります。

### Macでアプリケーションディレクトリにいれて、Terminalで実行する場合
/Applications/svelte-app-image-converter.app/Contents/MacOS/svelte-app-image-converter

##  Buy Me a Coffeeでサポート
https://www.buymeacoffee.com/terutoyoda

Copyright (c) 2024 TERU Inc.
