onload = (event) => {
    if (!document.getElementById('bus-demo-1')) {
        return;
    }

    AsciinemaPlayer.create('demos/intro.cast', document.getElementById('bus-demo-1'), {
        loop: true,
        theme: 'monokai'
    });
    AsciinemaPlayer.create('demos/intro.cast', document.getElementById('bus-demo-2'), {
        loop: true,
        theme: 'monokai'
    });
};