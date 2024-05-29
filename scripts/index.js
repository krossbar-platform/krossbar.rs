onload = (event) => {
    document.querySelectorAll('.demo-player').forEach(function (player) {
        let demo_path = 'demos/' + player.getAttribute("name") + ".cast";

        AsciinemaPlayer.create(demo_path, player, {
            loop: true,
            theme: 'monokai'
        });
    });

    hljs.highlightAll();
    hljs.initLineNumbersOnLoad();
};