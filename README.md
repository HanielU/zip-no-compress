This is a program that zips all folders in a directory without compressing the files. My usecase was to zip a folder of images for reading in a comic reader, but I didn't want to compress the images.

[This thread] also shows a way to do this with the `zip` command, but I wanted to make a program that would clearly do this for me without having to remember the flags and needing to write a shell script to loop through all the folders I needed to zip.

# Usage

```sh
$ cargo install --path . # Install the program
$ zip-no-compress <path_to_your_directory> # Zip all folders in the directory
```

[This thread]: https://superuser.com/questions/411394/zip-files-without-compression#:~:text=On%20OS%20X%2C%20zip%20%2DZ,and%20c%20without%20compressing%20them.&text=%40IanDunn's%20comment%20or%20%40JasomDotnet's%20answer,should%20be%20considered%20correct%20answers.
