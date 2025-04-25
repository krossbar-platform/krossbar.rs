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
        let isAnimating = false;
        let direction = 'next'; // Used to determine slide direction

        // Set initial state for carousel items
        items.forEach((item, i) => {
            if (i === 0) {
                item.classList.add('active');
                item.style.transform = 'translateX(0)';
                item.style.position = 'relative';
                item.style.display = 'block';
            } else {
                item.style.transform = 'translateX(100%)';
                item.style.position = 'absolute';
                item.style.display = 'none';
            }
        });

        // Create indicators
        if (indicatorsContainer) {
            for (let i = 0; i < totalItems; i++) {
                const indicator = document.createElement('div');
                indicator.classList.add('carousel-indicator');
                if (i === 0) indicator.classList.add('active');
                indicator.dataset.index = i;
                indicator.addEventListener('click', () => {
                    if (!isAnimating) {
                        // Set direction based on which indicator was clicked
                        direction = i > currentIndex ? 'next' : 'prev';
                        goToSlide(i);
                    }
                });
                indicatorsContainer.appendChild(indicator);
            }
        }

        // Previous button
        carousel.querySelector('.carousel-control-prev').addEventListener('click', () => {
            if (!isAnimating) {
                direction = 'prev';
                currentIndex = (currentIndex - 1 + totalItems) % totalItems;
                updateSlides();
            }
        });

        // Next button
        carousel.querySelector('.carousel-control-next').addEventListener('click', () => {
            if (!isAnimating) {
                direction = 'next';
                currentIndex = (currentIndex + 1) % totalItems;
                updateSlides();
            }
        });

        function goToSlide(index) {
            if (currentIndex === index) return;
            currentIndex = index;
            updateSlides();
        }

        function updateSlides() {
            if (isAnimating) return;
            isAnimating = true;

            // Find current active item
            const activeItem = carousel.querySelector('.carousel-item.active');
            if (!activeItem) {
                isAnimating = false;
                return;
            }

            // Get the new item to show
            const newActiveItem = items[currentIndex];

            // Position the new slide based on the direction
            if (direction === 'next') {
                // For "next", new slide comes from right
                newActiveItem.style.transform = 'translateX(100%)';
            } else {
                // For "prev", new slide comes from left
                newActiveItem.style.transform = 'translateX(-100%)';
            }

            // Make new item visible but positioned offscreen
            newActiveItem.style.display = 'block';
            newActiveItem.style.position = 'absolute';

            // Force reflow
            void newActiveItem.offsetWidth;

            // Set the direction for the current active item to exit
            if (direction === 'next') {
                // For "next", current slide exits to left
                activeItem.style.transform = 'translateX(-100%)';
                activeItem.classList.add('previous');
            } else {
                // For "prev", current slide exits to right
                activeItem.style.transform = 'translateX(100%)';
            }

            // Move new item to center (position 0)
            newActiveItem.style.transform = 'translateX(0)';

            // Wait for animation to finish
            setTimeout(() => {
                // Update classes and position
                activeItem.classList.remove('active', 'previous');
                activeItem.style.display = 'none';

                newActiveItem.classList.add('active');
                newActiveItem.style.position = 'relative';

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

                // Reinitialize content in the visible slide
                if (newActiveItem.querySelector('pre code')) {
                    hljs.highlightElement(newActiveItem.querySelector('pre code'));
                    if (window.hljs && window.hljs.lineNumbersBlock) {
                        hljs.lineNumbersBlock(newActiveItem.querySelector('pre code'));
                    }
                }

                // Reinitialize asciinema player in the visible slide
                const demoPlayer = newActiveItem.querySelector('.demo-player');
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

                // Reset animation flag
                isAnimating = false;

            }, 500); // Match this with the CSS transition duration
        }
    });
}