use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet};
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <meta name="description" content="Expert Fullstack Software Engineer specializing in modern web technologies"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options islands=true/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <html lang="en" class="scroll-smooth">
            <Stylesheet id="leptos" href="/pkg/webpage.css"/>
            <head>
                <meta charset="UTF-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
                <title>"Samantha Diaz - Marketing Specialist"</title>
            </head>
            <body class="bg-gradient-to-br from-portfolio-cream to-portfolio-yellow min-h-screen text-portfolio-brown">
                <Header/>
                <main class="py-12">
                    <div class="max-w-6xl mx-auto px-5">
                        <SummarySection/>
                        <ExperienceSection/>
                        <EducationSection/>
                        <SkillsSection/>
                        <ExtracurricularSection/>
                    </div>
                </main>
                <Footer/>
            </body>
        </html>
    }
}

#[component]
fn Header() -> impl IntoView {
    view! {
        <header class="bg-gradient-to-r from-portfolio-brown to-portfolio-brown/90 text-portfolio-cream py-8 relative overflow-hidden">
            <div class="absolute inset-0 opacity-10">
                <div class="absolute top-5 left-5 w-4 h-4 bg-portfolio-yellow rounded-full animate-float"></div>
                <div class="absolute top-20 right-20 w-3 h-3 bg-portfolio-sage rounded-full animate-float" style="animation-delay: 1s"></div>
                <div class="absolute bottom-10 left-1/3 w-2 h-2 bg-portfolio-yellow rounded-full animate-float" style="animation-delay: 2s"></div>
                <div class="absolute bottom-20 right-1/4 w-5 h-5 bg-portfolio-sage rounded-full animate-float" style="animation-delay: 3s"></div>
            </div>
            <div class="max-w-6xl mx-auto px-5 relative z-10">
                <div class="text-center">
                    <h1 class="text-5xl md:text-6xl font-bold mb-2 animate-slide-in-down drop-shadow-lg">
                        "SAMANTHA DIAZ"
                    </h1>
                    <p class="text-xl md:text-2xl text-portfolio-yellow mb-4 animate-slide-in-up">
                        "Marketing Specialist & Content Creator"
                    </p>
                    <div class="flex flex-col md:flex-row justify-center items-center gap-4 md:gap-8 animate-fade-in">
                        <ContactItem icon="📍" text="Toronto, ON"/>
                        <ContactItem icon="📞" text="(437) 439-4156"/>
                        <ContactItem icon="📧" text="sam.divs@gmail.com"/>
                        <ContactItem icon="💼" text="LinkedIn Profile"/>
                    </div>
                </div>
            </div>
        </header>
    }
}

#[component]
fn ContactItem(icon: &'static str, text: &'static str) -> impl IntoView {
    view! {
        <div class="flex items-center gap-2 transition-transform hover:-translate-y-1">
            <span class="text-lg">{icon}</span>
            <span class="text-sm md:text-base">{text}</span>
        </div>
    }
}

#[component]
fn SummarySection() -> impl IntoView {
    view! {
        <section class="bg-portfolio-cream/90 backdrop-blur-sm p-8 rounded-2xl shadow-xl border border-portfolio-sage/30 mb-8 transition-all hover:-translate-y-2 hover:shadow-2xl">
            <h2 class="text-3xl font-bold text-center mb-6 relative pb-2">
                "Professional Summary"
                <div class="absolute bottom-0 left-1/2 transform -translate-x-1/2 w-16 h-1 bg-gradient-to-r from-portfolio-sage to-portfolio-yellow rounded-full"></div>
            </h2>
            <div class="max-w-4xl mx-auto">
                <p class="text-lg text-center mb-8">
                    "Dynamic marketing professional with extensive experience in digital strategy, content creation, and audience engagement across diverse industries including sports, education, and technology."
                </p>
                <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                    <SummaryPoint
                        icon="🎯"
                        title="Strategic Marketing"
                        description="Comprehensive strategies for global brands with diverse audiences"
                    />
                    <SummaryPoint
                        icon="📱"
                        title="Social Media Expert"
                        description="KPI reporting, audience engagement, and digital analytics specialist"
                    />
                    <SummaryPoint
                        icon="🎬"
                        title="Content Creator"
                        description="Audiovisual content production and cross-functional project coordination"
                    />
                </div>
            </div>
        </section>
    }
}

#[component]
fn SummaryPoint(
    icon: &'static str,
    title: &'static str,
    description: &'static str,
) -> impl IntoView {
    view! {
        <div class="bg-gradient-to-br from-portfolio-yellow to-portfolio-sage p-6 rounded-xl text-center transition-transform hover:scale-105">
            <div class="text-2xl mb-2">{icon}</div>
            <h3 class="font-bold text-portfolio-brown mb-2">{title}</h3>
            <p class="text-sm text-portfolio-brown">{description}</p>
        </div>
    }
}

#[component]
fn ExperienceSection() -> impl IntoView {
    view! {
        <section class="bg-portfolio-cream/90 backdrop-blur-sm p-8 rounded-2xl shadow-xl border border-portfolio-sage/30 mb-8 transition-all hover:-translate-y-2 hover:shadow-2xl">
            <h2 class="text-3xl font-bold text-center mb-8 relative pb-2">
                "Professional Experience"
                <div class="absolute bottom-0 left-1/2 transform -translate-x-1/2 w-16 h-1 bg-gradient-to-r from-portfolio-sage to-portfolio-yellow rounded-full"></div>
            </h2>
            <div class="space-y-6">
                <ExperienceItem
                    title="Research Assistant"
                    company="Humber Polytechnic"
                    duration="Aug - Oct 2024"
                    tasks=vec![
                        "Developed social media strategies for knowledge dissemination and student recruitment",
                        "Produced audiovisual content for institutional social media platforms"
                    ]
                />
                <ExperienceItem
                    title="Research Analyst Intern"
                    company="Genie Technologies"
                    duration="Apr - Jun 2024"
                    tasks=vec![
                        "Implemented quantitative and qualitative market research for emerging ed-tech",
                        "Designed and implemented various methods to gather and analyze primary data"
                    ]
                />
                <ExperienceItem
                    title="Social Media Manager"
                    company="Major League Baseball"
                    duration="Jan 2017 - Jan 2023"
                    tasks=vec![
                        "Led social media content strategy across Instagram, Facebook and X",
                        "Grew thousands of followers while maintaining competitive engagement",
                        "Collaborated in quantitative and qualitative market research of the sports industry in Mexico",
                        "Developed digital strategies including Google Ads campaigns and analytics reporting"
                    ]
                />
                <ExperienceItem
                    title="Community Manager"
                    company="Nurun - Publicis Groupe"
                    duration="Aug 2015 - Jan 2017"
                    tasks=vec![
                        "Implemented social media strategy for top national brands",
                        "Created periodical digital analytics reports for stakeholders"
                    ]
                />
                <ExperienceItem
                    title="Marketing Specialist"
                    company="Tecnológico de Monterrey"
                    duration="Sep 2013 - Oct 2014"
                    tasks=vec![
                        "Promoted education sales and proposed strategies to reach new generations",
                        "Coordinated media-buy for printed and radio advertisement",
                        "Planned school events"
                    ]
                />
            </div>
        </section>
    }
}

#[component]
fn ExperienceItem(
    title: &'static str,
    company: &'static str,
    duration: &'static str,
    tasks: Vec<&'static str>,
) -> impl IntoView {
    view! {
        <div class="bg-portfolio-yellow/20 p-6 rounded-xl border-l-4 border-portfolio-sage transition-all hover:border-portfolio-yellow hover:translate-x-2">
            <div class="flex flex-col md:flex-row md:justify-between md:items-center mb-4 gap-2">
                <div>
                    <h3 class="text-xl font-bold text-portfolio-brown">{title}</h3>
                    <p class="text-portfolio-sage font-semibold">{company}</p>
                </div>
                <span class="bg-portfolio-sage text-portfolio-brown px-3 py-1 rounded-full text-sm font-medium self-start">
                    {duration}
                </span>
            </div>
            <ul class="space-y-2">
                {tasks.into_iter().map(|task| view! {
                    <li class="flex items-start gap-2">
                        <span class="text-portfolio-sage">{"▶"}</span>
                        <span class="text-portfolio-brown">{task}</span>
                    </li>
                }).collect_view()}
            </ul>
        </div>
    }
}

#[component]
fn EducationSection() -> impl IntoView {
    view! {
        <section class="bg-portfolio-cream/90 backdrop-blur-sm p-8 rounded-2xl shadow-xl border border-portfolio-sage/30 mb-8 transition-all hover:-translate-y-2 hover:shadow-2xl">
            <h2 class="text-3xl font-bold text-center mb-8 relative pb-2">
                "Education & Certifications"
                <div class="absolute bottom-0 left-1/2 transform -translate-x-1/2 w-16 h-1 bg-gradient-to-r from-portfolio-sage to-portfolio-yellow rounded-full"></div>
            </h2>
            <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
                <EducationItem
                    degree="Research Analyst Graduate Program"
                    institution="Humber College, 2024"
                    note="IGNITE Scholarship recipient • Graduated with Honours"
                />
                <EducationItem
                    degree="Marketing Master's Degree"
                    institution="Tecnológico de Monterrey, 2019"
                    note="4.0 GPA"
                />
                <EducationItem
                    degree="Bachelor's in Communication Sciences"
                    institution="Tecnológico de Monterrey, 2012"
                    note="Graduated with Honours"
                />
            </div>
            <div>
                <h3 class="text-xl font-bold text-center text-portfolio-brown mb-4">"Professional Certifications"</h3>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                    <EducationItem
                        degree="TCPS2 Certificate of Completion"
                        institution="Panel on Research Ethics, 2023"
                        note=""
                    />
                    <EducationItem
                        degree="Search Engine Optimization Certificate"
                        institution="Coursera, 2020"
                        note=""
                    />
                </div>
            </div>
        </section>
    }
}

#[component]
fn EducationItem(
    degree: &'static str,
    institution: &'static str,
    note: &'static str,
) -> impl IntoView {
    view! {
        <div class="bg-gradient-to-br from-portfolio-sage to-portfolio-yellow p-6 rounded-xl text-center transition-transform hover:scale-105">
            <h4 class="font-bold text-portfolio-brown mb-2">{degree}</h4>
            <p class="text-portfolio-brown font-semibold mb-2">{institution}</p>
            {if !note.is_empty() {
                view! {
                    <p class="text-sm text-portfolio-brown">{note}</p>
                }.into_any()
            } else {
                view! {None}.into_any()
            }}
        </div>
    }
}

#[component]
fn SkillsSection() -> impl IntoView {
    view! {
        <section class="bg-portfolio-cream/90 backdrop-blur-sm p-8 rounded-2xl shadow-xl border border-portfolio-sage/30 mb-8 transition-all hover:-translate-y-2 hover:shadow-2xl">
            <h2 class="text-3xl font-bold text-center mb-8 relative pb-2">
                "Core Competencies"
                <div class="absolute bottom-0 left-1/2 transform -translate-x-1/2 w-16 h-1 bg-gradient-to-r from-portfolio-sage to-portfolio-yellow rounded-full"></div>
            </h2>
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8">
                <SkillCategory
                    icon="🎯"
                    title="Marketing & Strategy"
                    skills=vec!["Digital Marketing", "Google Ads", "Market Research", "Analytics"]
                />
                <SkillCategory
                    icon="📱"
                    title="Social Media"
                    skills=vec!["Content Strategy", "Community Management", "KPI Reporting", "Engagement"]
                />
                <SkillCategory
                    icon="🎬"
                    title="Content Creation"
                    skills=vec!["Audiovisual Production", "Content Planning", "Brand Storytelling", "SEO"]
                />
                <SkillCategory
                    icon="🗣️"
                    title="Languages"
                    skills=vec!["English", "Spanish", "French"]
                />
            </div>
        </section>
    }
}

#[component]
fn SkillCategory(
    icon: &'static str,
    title: &'static str,
    skills: Vec<&'static str>,
) -> impl IntoView {
    view! {
        <div class="text-center">
            <div class="text-2xl mb-2">{icon}</div>
            <h3 class="font-bold text-portfolio-brown mb-4 text-lg">{title}</h3>
            <div class="flex flex-wrap gap-2 justify-center">
                {skills.into_iter().map(|skill| view! {
                    <span class="bg-portfolio-sage text-portfolio-brown px-3 py-1 rounded-full text-sm font-medium transition-all hover:bg-portfolio-yellow hover:-translate-y-1">
                        {skill}
                    </span>
                }).collect_view()}
            </div>
        </div>
    }
}

#[component]
fn ExtracurricularSection() -> impl IntoView {
    view! {
        <section class="bg-portfolio-cream/90 backdrop-blur-sm p-8 rounded-2xl shadow-xl border border-portfolio-sage/30 mb-8 transition-all hover:-translate-y-2 hover:shadow-2xl">
            <h2 class="text-3xl font-bold text-center mb-8 relative pb-2">
                "Beyond Professional Life"
                <div class="absolute bottom-0 left-1/2 transform -translate-x-1/2 w-16 h-1 bg-gradient-to-r from-portfolio-sage to-portfolio-yellow rounded-full"></div>
            </h2>
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                <ActivityCard
                    title="🤖 FIRST Canada Volunteer | 2025"
                    description="On-stage staff at international youth robotics competition"
                />
                <ActivityCard
                    title="🏫 Humber College Volunteer | 2023-2024"
                    description="Event collaboration: Walk for Reconciliation, Research Analyst Annual Forum, IC Hub events"
                />
                <ActivityCard
                    title="🎓 Student Advisory Committee | 2023"
                    description="Contributed feedback and proposals for enhancing student college experience"
                />
                <ActivityCard
                    title="📣 All-Star Cheerleader | 2008-2017"
                    description="Competed in multiple national championships • Cheer captain"
                />
                <ActivityCard
                    title="🏕️ Summer Camp Volunteer | 2011-2012"
                    description="Motivated kids' reading habits • Taught short film creation course"
                />
            </div>
        </section>
    }
}

#[component]
fn ActivityCard(title: &'static str, description: &'static str) -> impl IntoView {
    view! {
        <div class="bg-portfolio-sage/30 p-6 rounded-xl border-2 border-portfolio-sage transition-all hover:border-portfolio-yellow hover:-translate-y-1">
            <h4 class="font-bold text-portfolio-brown mb-2">{title}</h4>
            <p class="text-portfolio-brown text-sm">{description}</p>
        </div>
    }
}

#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer class="bg-portfolio-brown text-portfolio-cream text-center py-8 mt-12">
            <div class="max-w-6xl mx-auto px-5">
                <p>"© 2025 Samantha Diaz. Ready to create compelling content and drive marketing success."</p>
            </div>
        </footer>
    }
}
