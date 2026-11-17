import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.llm.keyword.LlmKeywords as LLM
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

String loginUrl  = 'http://localhost/e-pkl/siswa-web/public/login/'

String usernameSpasi = '    8888888801'          // sengaja diawali spasi

TestObject inputUser = findTestObject('Login Siswa/Page_Masuk - Pantau PKL/input_Masukkan NISN, email, atau username')

TestObject inputPass = findTestObject('Login Siswa/Page_Masuk - Pantau PKL/input_Masukkan password')

TestObject btnLogin  = findTestObject('Login Siswa/Page_Masuk - Pantau PKL/button_loginSubmit')

WebUI.openBrowser('')

WebUI.maximizeWindow()

WebUI.navigateToUrl(loginUrl)

WebUI.waitForElementVisible(inputUser, 10)

WebUI.setText(inputUser, usernameSpasi)

WebUI.setEncryptedText(inputPass, 'RigbBhfdqOCS7dY3cpR0kg==')

String nilaiInput = WebUI.getAttribute(inputUser, 'value')

WebUI.verifyEqual(nilaiInput, usernameSpasi, FailureHandling.STOP_ON_FAILURE)

WebUI.click(btnLogin)

WebUI.delay(2)

String urlSekarang = WebUI.getUrl()

WebUI.verifyMatch(urlSekarang, '.*/login/?.*', true, FailureHandling.STOP_ON_FAILURE)

WebUI.verifyElementPresent(btnLogin, 5, FailureHandling.STOP_ON_FAILURE)

WebUI.closeBrowser()