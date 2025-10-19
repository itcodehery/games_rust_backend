import pandas as pd


def main():
    df = pd.read_excel("game_ranks.xlsx")
    print(df.columns)
    df.to_csv("games_dataset.csv")
    print(df['Genre'].unique)


if __name__ == "__main__":
    main()
