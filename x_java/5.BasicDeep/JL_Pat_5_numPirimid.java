import java.util.*;

public class JL_Pat_5_numPirimid {

  public static void main(String[] args) {
    Scanner sc = new Scanner(System.in);

    System.out.println("INPUT:");
    int lines = sc.nextInt();
    int n = 0;

    for (int i = 0; i < lines; i++) {
      for (int j = 0; j < i; j++) {
        System.out.print(n);
        n++;
      }
      System.out.println();
    }
    sc.close();
  }
}
