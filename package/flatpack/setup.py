from setuptools import setup, find_packages

with open("README.md", "r") as f:
    long_description = f.read()

setup(
    name="flatpack",
    version="0.0.5",
    packages=find_packages(),
    install_requires=[
        "requests"
    ],
    author="Romlin Group AB",
    author_email="hello@romlin.com",
    description="Train AI models - not your patience",
    long_description=long_description,
    long_description_content_type="text/markdown",
    entry_points={
        "console_scripts": [
            "flatpack=flatpack.main:main"
        ],
    }
)
