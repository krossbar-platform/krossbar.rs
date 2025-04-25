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

    // Initialize carousels
    initCarousels();
});

function initCarousels() {
    document.querySelectorAll('.demo-carousel').forEach(carousel => {
        const items = carousel.querySelectorAll('.carousel-item');
        const indicatorsContainer = carousel.querySelector('.carousel-indicators');
        const totalItems = items.length;
        let currentIndex = 0;

        // Create indicators
        if (indicatorsContainer) {
            for (let i = 0; i < totalItems; i++) {
                const indicator = document.createElement('div');
                indicator.classList.add('carousel-indicator');
                if (i === 0) indicator.classList.add('active');
                indicator.dataset.index = i;
                indicator.addEventListener('click', () => {
                    goToSlide(i);
                });
                indicatorsContainer.appendChild(indicator);
            }
        }

        // Previous button
        carousel.querySelector('.carousel-control-prev').addEventListener('click', () => {
            currentIndex = (currentIndex - 1 + totalItems) % totalItems;
            updateSlides();
        });

        // Next button
        carousel.querySelector('.carousel-control-next').addEventListener('click', () => {
            currentIndex = (currentIndex + 1) % totalItems;
            updateSlides();
        });

        function goToSlide(index) {
            currentIndex = index;
            updateSlides();
        }

        function updateSlides() {
            // Update items
            items.forEach((item, i) => {
                if (i === currentIndex) {
                    item.classList.add('active');
                } else {
                    item.classList.remove('active');
                }
            });

            // Update indicators
            if (indicatorsContainer) {
                const indicators = indicatorsContainer.querySelectorAll('.carousel-indicator');
                indicators.forEach((indicator, i) => {
                    if (i === currentIndex) {
                        indicator.classList.add('active');
                    } else {
                        indicator.classList.remove('active');
                    }
                });
            }

            // Reinitialize code highlighting in the visible slide
            const activeSlide = items[currentIndex];
            if (activeSlide.querySelector('pre code')) {
                hljs.highlightElement(activeSlide.querySelector('pre code'));
                if (window.hljs && window.hljs.lineNumbersBlock) {
                    hljs.lineNumbersBlock(activeSlide.querySelector('pre code'));
                }
            }

            // Reinitialize asciinema player in the visible slide
            const demoPlayer = activeSlide.querySelector('.demo-player');
            if (demoPlayer && !demoPlayer.querySelector('.ap-terminal')) {
                let demo_path = 'demos/' + demoPlayer.getAttribute("name") + ".cast";
                AsciinemaPlayer.create(demo_path, demoPlayer, {
                    loop: true,
                    theme: 'krossbar',
                    terminalFontFamily: "ui-monospace,SFMono-Regular,Menlo,Monaco,Consolas,\"Liberation Mono\",\"Courier New\",monospace",
                    rows: 22,
                    cols: 86,
                    terminalFontSize: "16px",
                    fit: false,
                    poster: 'npt:0:1'
                });
            }
        }
    });
}