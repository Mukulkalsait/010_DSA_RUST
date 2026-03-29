import java.util.*;

public class M10_assigement {

  public static int que1(Scanner sc) {
    System.out.println("Please inter A:");
    int a = sc.nextInt();
    System.out.println("Please inter B:");
    int b = sc.nextInt();
    System.out.println("Please inter C:");
    int c = sc.nextInt();
    return (a + b + c) / 3;

  }

  public static String que2(Scanner sc) {
    System.out.println("Please Inter number for checking if its Even or Odd:");
    int num = sc.nextInt();
    return (num % 2 == 0) ? num + "is Even Number." : num + "is Odd Number.";
  }

  public static boolean palendromecheck(int num) {
    int forword = num;
    int reverse = 0;
    while (forword > 0) {
      reverse = reverse * 10 + (forword % 10);
      forword = forword / 10;
    }
    System.out.println(forword); // 0
    System.out.println(reverse); // 8
    System.out.println(num); // 112211
    return (num == reverse) ? true : false;
  }

  public static void que3(Scanner sc) {
    System.out.println("Please Inter number for checking if its Palendrome or not :");
    int num = sc.nextInt();
    if (palendromecheck(num)) {
      System.out.println(num + " is a Palendrome Number.");
    } else {
      System.out.println(num + " is NOT a Palendrome Number.");
    }
  }

  public static void que4() {
    System.out.println(" Please check and learn about the Followings:");
    System.out.println(
        " \n Math.IEEEremainder(f1, f2) \n Math.abs(a) \n Math.absExact(a) \n Math.ceil(a) \n Math.addExact(x, y) \n Math.cos(a) \n Math.floor(a) \n Math.incrementExact(a) \n Math.max(a, b) \n Math.min(a, b) \n Math.sqrt(a) \n Math.avg() \n Math.pow(a, b)");
  }

  public static int que5(Scanner sc) {
    System.out.println("Please Inter number to know the sum of its each numbers:");
    int num = sc.nextInt();
    int sum = 0;
    while (num > 0) {
      sum = sum + (num % 10);
      num = num / 10;
    }
    return sum;
  }

  public static void queSelector(Scanner sc, int que) {

    switch (que) {
      case 1:
        System.out.println("The Average of the Three numbers Above is : " + que1(sc));
        break;

      case 2:
        System.out.println(que2(sc));
        break;

      case 3:
        que3(sc);
        break;

      case 4:
        que4();
        break;

      case 5:
        System.out.println(que5(sc));
        break;

      default:
        break;
    }
  }

  public static void numCheaker(Scanner sc) {

    System.out.println("Please select which Quetion you want to solve:");
    System.out.println();
    System.out.println(
        "1) 3 number avg \n2) OddEven Method \n3) Palendrome Cheacker \n4) Math.<functions>\n5) sum of digits of Numbers: \n\n Options(1,2,3,4,5)");
    int que = sc.nextInt();
    if (1 <= que && que < 6) {
      queSelector(sc, que);
    } else {
      System.out.println("Invaid Output !");
      System.out.println("Please try again:");
      numCheaker(sc);
    }
    System.out.println();
    System.out.println("Do you want to check another quetion? \n \n1) Yes \n 2) NO\n\n Options(1,2,)");
    int repeater = sc.nextInt();
    if (1 <= repeater && repeater < 3) {
      if (repeater == 1) {
        numCheaker(sc);
      } else {
        System.out.println("End of Assigement.");
      }
    } else {
      System.out.println("Invaid Output !");
      System.out.println("End of Assigement.");
    }
  }

  public static void main(String[] args) {
    Scanner sc = new Scanner(System.in);
    numCheaker(sc);
    sc.close();
  }
}
