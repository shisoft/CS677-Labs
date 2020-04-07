import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.net.HttpURLConnection;
import java.net.URL;
import java.net.URLConnection;
import java.util.concurrent.atomic.AtomicLong;
import java.util.logging.FileHandler;
import java.util.logging.Logger;
import java.util.logging.SimpleFormatter;

import static spark.Spark.*;

public class Main {


    private final static java.util.logging.Logger logger = Logger.getLogger(Main.class.getName());
    private static FileHandler fh;
    //private static Logger logger = LoggerFactory.getLogger(Main.class);

    public static void main(String[] args) throws IOException {

        fh = new FileHandler("FRONT.log");
        logger.addHandler(fh);
        SimpleFormatter formatter = new SimpleFormatter();
        fh.setFormatter(formatter);
        AtomicLong ct = new AtomicLong();
        port(34841);
         get("/search/:topic", (req,res)->{

             long startTime = System.currentTimeMillis();
            logger.info("search for topic");
            logger.info("topic"+req.params(":topic"));
             String re = search(req.params(":topic"));
             long endTime = System.currentTimeMillis();
             ct.addAndGet((endTime - startTime));
             logger.info(ct+"tim!!!!se");
             return re;

         });
        get("/lookup/:id", (req,res)->{
            long startTime = System.currentTimeMillis();

            logger.info("look up for id");
            logger.info("id"+req.params(":id"));
            String re = lookup(req.params(":id"));

            long endTime = System.currentTimeMillis();
            ct.addAndGet((endTime - startTime));
            logger.info(ct+"tim!!!!look");
            return re;

        });
        put("/buy/:id", (req,res)->{
            long startTime = System.currentTimeMillis();

            logger.info("buy");
            logger.info("id"+req.params(":id"));
            String re = buy(req.params(":id"));

            long endTime = System.currentTimeMillis();
            ct.addAndGet((endTime - startTime));
            logger.info(ct+"tim!!!!buy");
            return re;
        });


    }

    private static String lookup(String params) throws IOException {

        URL request2 = new URL("http://128.119.243.164:34842/lookup/"+params);
        logger.info("look up "+params);
        URLConnection yc2 = request2.openConnection();
        BufferedReader r = new BufferedReader(new InputStreamReader(
                yc2.getInputStream()));
        String inputLine3;
        StringBuilder sb = new StringBuilder();
        while ((inputLine3 = r.readLine()) != null)
            sb.append(inputLine3);
        r.close();
        logger.info("Sever returns: "+sb.toString() );
        return sb.toString();

    }

    private static String search(String params) throws IOException {
        URL request2 = new URL("http://128.119.243.164:34842/search/"+params);
        logger.info("search "+params);
        URLConnection yc2 = request2.openConnection();
        BufferedReader r = new BufferedReader(new InputStreamReader(
                yc2.getInputStream()));
        String inputLine3;
        StringBuilder sb = new StringBuilder();
        while ((inputLine3 = r.readLine()) != null)
            sb.append(inputLine3);
        r.close();
        logger.info("Sever returns: "+sb.toString() );
        return sb.toString();


    }

    public static String buy(String pa) throws IOException {

        URL request2 = new URL("http://128.119.243.168:34843/buy/"+pa);
        logger.info("Tring to buy "+pa);

        URLConnection yc2 = request2.openConnection();
        HttpURLConnection http = (HttpURLConnection)yc2;
        http.setRequestMethod("PUT");
        http.setDoOutput(true);

        BufferedReader r = new BufferedReader(new InputStreamReader(
                yc2.getInputStream()));

        String inputLine3;
        StringBuilder sb = new StringBuilder();

        while ((inputLine3 = r.readLine()) != null)
            sb.append(inputLine3);

        r.close();

        logger.info("Sever returns: "+sb.toString() );

        return sb.toString();

    }


}