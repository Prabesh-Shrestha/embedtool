<div class="footer">
  hello
  <div class="utils">
    <button class="compile">Compile</button>
    <button class="upload">Upload</button>
  </div>
</div>

<style>
    * {
        font-size: 20px;
    }
  .footer {
    background-color: rgb(88, 85, 85);
    padding: 20px 0px;
    width: 100%;
    position: absolute;
    bottom: 0;
    left: 0;
    display: flex;
    justify-content: space-between;
  }
  .footer .utils .upload {
    background-color: aquamarine;
    padding: 5px 10px;
    margin-right: 15px;
    color: black;
    border-radius: 20px;
    border: 0px;
  }


  .footer .utils .upload:active {
    opacity: 0.6;
    transition: opacity 5ms ease-in;
  }

  .footer .utils .compile {
    background-color: rgb(192, 192, 192);
    padding: 5px 10px;
    margin-right: 20px;
    color: black;
    border-radius: 5px;
    border: 0px;
  }

  .footer .utils .compile:active {
    opacity: 0.6;
    transition: opacity 5ms ease-in;
  }
</style>
