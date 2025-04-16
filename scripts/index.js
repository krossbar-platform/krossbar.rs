document.addEventListener('DOMContentLoaded', function () {
    // Initialize demos
    document.querySelectorAll('.demo-player').forEach(function (player) {
        let demo_path = 'demos/' + player.getAttribute("name") + ".cast";

        AsciinemaPlayer.create(demo_path, player, {
            loop: true,
            theme: 'krossbar',
            terminalFontFamily: "ui-monospace,SFMono-Regular,Menlo,Monaco,Consolas,\"Liberation Mono\",\"Courier New\",monospace",
            rows: 22,
            cols: 86,
            terminalFontSize: "16px",
            fit: false,
            poster: 'npt:0:1'
        });
    });

    // Initialize code highlighting
    hljs.highlightAll();
    hljs.initLineNumbersOnLoad();

    // Smooth scrolling for anchor links
    document.querySelectorAll('a[href^="#"]').forEach(anchor => {
        anchor.addEventListener('click', function (e) {
            e.preventDefault();
            const targetId = this.getAttribute('href');
            const targetElement = document.querySelector(targetId);

            if (targetElement) {
                const headerOffset = 80;
                const elementPosition = targetElement.getBoundingClientRect().top;
                const offsetPosition = elementPosition + window.pageYOffset - headerOffset;

                window.scrollTo({
                    top: offsetPosition,
                    behavior: 'smooth'
                });
            }
        });
    });

    // Add animation to feature cards on scroll
    const observerOptions = {
        root: null,
        rootMargin: '0px',
        threshold: 0.1
    };

    const observer = new IntersectionObserver((entries) => {
        entries.forEach(entry => {
            if (entry.isIntersecting) {
                entry.target.classList.add('animate-in');
                observer.unobserve(entry.target);
            }
        });
    }, observerOptions);

    document.querySelectorAll('.feature-card, .feature-item, .demo-block').forEach(element => {
        observer.observe(element);
    });
});