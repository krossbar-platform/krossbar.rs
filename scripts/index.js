onload = (event) => {
    document.querySelectorAll('.demo-player').forEach(function (player) {
        let demo_path = 'demos/' + player.getAttribute("name") + ".cast";

        AsciinemaPlayer.create(demo_path, player, {
            loop: true,
            theme: 'krossbar',
            terminalFontFamily: "ui-monospace,SFMono-Regular,Menlo,Monaco,Consolas,\"Liberation Mono\",\"Courier New\",monospace",
            rows: 22,
            cols: 86,
            terminalFontSize: "16px",
            fit: false
        });
    });

    hljs.highlightAll();
    hljs.initLineNumbersOnLoad();
};