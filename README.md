# webp/avif Image Converter for Mac

<img src="./app-icon.png" alt="webp/avif Image Converter" width="200"/><br>
[日本語版はこちら](./README_ja.md)

## Function

- webp/avif Image Converter is a desktop application for converting image files to webp and avif formats. This application is developed using Svelte and Tauri.

## Installation

To install webp/avif Image Converter, follow these steps

1. download the latest release. 
2. Unzip the downloaded file and run the application. 3.
3. run npm run build, npm run tauri:build to build the application. 
4. When the build is complete, the application will be generated in the src-tauri/target/bundle/macos directory.

## Usage

1. Open the application. 
2. Click the "Select Output Directory" button to select the destination for the converted image files. 
3. Drag and drop the image files you wish to convert into the window or add files via the "Select File" button. \
4. Select the desired output format (webp or avif) from the drop-down menu at the bottom.
5. click the "Convert Images" button to begin conversion.

## Notes

1. we have tested this application on Mac only. 
2. we are not responsible for any direct or indirect damages, including but not limited to data loss or functional failure, related to the use of this application. 
3. the user is responsible for taking care not to infringe on the rights of any third party by using this application. Any and all liability related to infringement of third party rights belongs to the user.

## Miscellaneous

If you run the application from the GUI and it does not convert well, it is possible that write permission to the file system has not been reliably granted with this execution method. If this is the case, running the application from the Terminal may alleviate certain security restrictions.

### If you put the file in the application directory on a Mac and run it from the Terminal.
/Applications/svelte-app-image-converter.app/Contents/MacOS/svelte-app-image-converter

## Supported by Buy Me a Coffee
https://www.buymeacoffee.com/terutoyoda

Copyright (c) 2024 TERU Inc.